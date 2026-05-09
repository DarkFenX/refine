use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared::{process_output_of_cycle_with_cutoff, process_output_of_lls_cseq_with_cutoff},
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
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Local effects, considers only part of sequence until charges are out
#[must_use]
pub(in crate::svc::vast) fn aggr_local_clip<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    match cseq.get_hard_dt().is_some() {
        // Consider hard downtime as end of clip
        true => process_hard_dt(ctx, calc, item_uid, cseq, ospec, accum, inv_local),
        false => process_regular(ctx, calc, item_uid, cseq, ospec, accum, inv_local),
    }
}

fn process_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
) -> bool
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_local_output(
            ctx,
            calc,
            item_uid,
            ospec,
            &inv_local,
            cycle_part.data.active.chargedness,
        );
        match cycle_part.data.dt_soft {
            // Add first cycle after which there is a reload
            Some(soft_dt) if soft_dt.reason.reload => {
                reload = true;
                accum.add_output_full(&cycle_output, None, Count::ONE);
                // Record only active duration before reload, ignore soft downtime duration
                accum.time += cycle_part.data.active.duration;
                break;
            }
            _ => {
                let part_cycle_count = match cycle_part.repeat_count {
                    InfCount::Count(part_cycle_count) => part_cycle_count,
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return false,
                };
                if part_cycle_count > Count::ZERO {
                    accum.add_output_full(&cycle_output, None, part_cycle_count);
                    accum.time += cycle_part.data.get_main_duration() * part_cycle_count.into_pvalue();
                }
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}

fn process_hard_dt<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
) -> bool
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    match cseq {
        // Infinite cycle with hard downtime on every cycle means we have just that cycle in clip
        CycleSeq::Inf(inner) => {
            let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
            let inner_conv = inner.convert_with(&mut converter);
            process_output_of_cycle_with_cutoff(&mut accum.instances, &inner_conv.data, None, Count::ONE);
            // Record time until reload or hard downtime starts
            match inner.data.dt_soft {
                Some(soft_dt) if soft_dt.reason.reload => accum.time += inner.data.active.duration,
                _ => accum.time += inner_conv.data.cycle_main_duration,
            }
            true
        }
        CycleSeq::LoopLimSin(inner) => {
            if let Some(soft_dt) = inner.p1_data.dt_soft
                && soft_dt.reason.reload
            {
                // Case when there is a reload right after first cycle
                let output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p1_data.active.chargedness);
                let loop_inner_duration = inner.get_inner_duration();
                match inv_local.get_output_completion_duration() > loop_inner_duration {
                    true => accum.add_output_time_limited(&output, None, Count::ONE, loop_inner_duration.into_value()),
                    false => accum.add_output_full(&output, None, Count::ONE),
                }
                // Stop counting time at reload, after active cycle is finished
                accum.time += inner.p1_data.active.duration;
            } else {
                // Case when all sequence cycles are allowed to run, possibly with reload after the
                // last cycle
                let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
                let inner_conv = inner.convert_with(&mut converter);
                process_output_of_lls_cseq_with_cutoff(&mut accum.instances, &inner_conv, None, Count::ONE);
                // Record time until reload or hard downtime starts
                match inner.p2_data.dt_soft {
                    Some(soft_dt) if soft_dt.reason.reload => {
                        accum.time += inner.p1_data.get_main_duration() * inner.p1_repeat_count.into_pvalue()
                            + inner.p2_data.active.duration;
                    }
                    _ => {
                        accum.time += inner.p1_data.get_main_duration() * inner.p1_repeat_count.into_pvalue()
                            + inner.p2_data.get_main_duration();
                    }
                }
            }
            true
        }
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}
