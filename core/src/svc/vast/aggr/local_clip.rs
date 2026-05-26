use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared::{
        AggrHardDtSimple, AggrPartData, AggrPartDataTail, process_output_of_cycle_with_cutoff,
        process_output_of_lls_with_cutoff,
    },
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    misc::InfCount,
    nd::NEffectOutputGetter,
    num::{Count, PValue},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CSeqLoopLimSin, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::LibConverter,
};

// Local effects, considers only part of sequence until charges are out
#[must_use]
pub(in crate::svc::vast) fn aggr_local_clip<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    match cseq.get_hard_dt().is_some() {
        // Consider hard downtime as end of clip
        true => process_hard_dt(cseq, accum, converter),
        false => process_regular(cseq, accum, converter),
    }
}

fn process_regular<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    accum: &mut SeqAccum<IA>,
    mut converter: C,
) -> bool
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>,
{
    let mut reload = false;
    let cseq_parts = cseq.get_cseq_parts();
    for cseq_part in cseq_parts.iter() {
        match cseq_part.data.soft_dt {
            // Add first cycle after which there is a reload
            Some(soft_dt) if soft_dt.reason.reload => {
                reload = true;
                let cseq_part_data_conv = converter.lib_convert(cseq_part.data);
                accum.add_output_full(&cseq_part_data_conv.output, None, Count::ONE);
                // Record only active duration before reload, ignore soft downtime duration
                accum.time += cseq_part.data.active.duration;
                break;
            }
            _ => {
                let part_cycle_count = match cseq_part.repeat_count {
                    InfCount::Count(part_cycle_count) => part_cycle_count,
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return false,
                };
                if part_cycle_count > Count::ZERO {
                    let cseq_part_data_conv = converter.lib_convert(cseq_part.data);
                    accum.add_output_full(&cseq_part_data_conv.output, None, part_cycle_count);
                    accum.time += cseq_part_data_conv.cycle_main_duration * part_cycle_count.into_pvalue();
                }
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cseq_parts.loops || reload
}

fn process_hard_dt<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    accum: &mut SeqAccum<IA>,
    mut converter: C,
) -> bool
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match cseq {
        // Infinite cycle with hard downtime on every cycle means we have just that cycle in clip
        CycleSeq::Inf(inner) => {
            let inner_data_conv = converter.lib_convert(inner.data);
            process_output_of_cycle_with_cutoff(&mut accum.instances, &inner_data_conv, None, Count::ONE);
            // Record time until reload or hard downtime starts
            let p1_final_cycle_duration = match inner.data.soft_dt {
                Some(soft_dt) if soft_dt.reason.reload => inner.data.active.duration,
                _ => inner_data_conv.cycle_main_duration,
            };
            accum.time += p1_final_cycle_duration;
            true
        }
        CycleSeq::LoopLimSin(inner) => {
            if let Some(soft_dt) = inner.p1_data.soft_dt
                && soft_dt.reason.reload
            {
                // Case when there is a reload right after first cycle
                let inner_p1_data_conv: AggrPartData<_> = converter.lib_convert(inner.p1_data);
                let loop_inner_duration = inner_p1_data_conv
                    .cycle_main_duration
                    .mul_add(inner.p1_repeat_count.into_pvalue(), inner.p2_data.get_main_duration());
                match inner_p1_data_conv.output.get_completion_duration() > loop_inner_duration {
                    true => accum.add_output_time_limited(
                        &inner_p1_data_conv.output,
                        None,
                        Count::ONE,
                        loop_inner_duration.into_value(),
                    ),
                    false => accum.add_output_full(&inner_p1_data_conv.output, None, Count::ONE),
                }
                // Stop counting time at reload, after active cycle is finished
                accum.time += inner.p1_data.active.duration;
            } else {
                // Case when all sequence cycles are allowed to run, possibly with reload after the
                // last cycle
                let inner_conv: CSeqLoopLimSin<_, AggrHardDtSimple> = inner.convert_with(&mut converter);
                process_output_of_lls_with_cutoff(&mut accum.instances, &inner_conv, None, Count::ONE);
                // Record time until reload or hard downtime starts
                let p2_final_cycle_duration = match inner.p2_data.soft_dt {
                    Some(soft_dt) if soft_dt.reason.reload => inner.p2_data.active.duration,
                    _ => inner_conv.p2_data.cycle_main_duration,
                };
                accum.time += inner_conv
                    .p1_data
                    .cycle_main_duration
                    .mul_add(inner_conv.p1_repeat_count.into_pvalue(), p2_final_cycle_duration);
            }
            true
        }
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}
