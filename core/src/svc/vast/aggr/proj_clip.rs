use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverter, get_proj_spool_cycle_output,
        process_output_for_lls_cseq_spool_hard_dt,
    },
    shared::{AggrPartData, AggrPartDataSpool, AggrPartDataSpoolTail, AggrPartDataTail},
    shared_clip::{aclip_process_both_for_cseq_hard_dt, aclip_process_both_for_cseq_regular},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    misc::InfCount,
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::LibConverter,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc::vast) fn aggr_proj_clip<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: SeqAccum<IA>,
) -> Option<SeqAccum<IA>>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)?;
    let inv_spool = AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec);
    let converter = ProjConverter::new(ctx, calc, projector_uid, ospec, &inv_proj);
    match (inv_spool, cseq.get_hard_dt().is_some()) {
        (Some(inv_spool), true) => {
            aclip_process_both_for_cseq_spool_hard_dt(cseq, &inv_proj, &inv_spool, accum, converter)
        }
        (Some(inv_spool), false) => aclip_process_both_for_cseq_spool(cseq, &inv_proj, &inv_spool, accum, converter),
        (None, true) => aclip_process_both_for_cseq_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
        (None, false) => aclip_process_both_for_cseq_regular(cseq, inv_proj.chance_mult, accum, converter),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aclip_process_both_for_cseq_spool<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    mut accum: SeqAccum<IA>,
    mut converter: C,
) -> Option<SeqAccum<IA>>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartDataSpool>,
{
    let mut uninterrupted_cycles = Count::ZERO;
    let cseq_parts = cseq.get_cseq_parts();
    'part: for cseq_part in cseq_parts.iter() {
        let cseq_part_data_conv = converter.lib_convert(cseq_part.data);
        // Add first cycle after which there is a reload. Here we assume every part has 1+ cycle
        // count, which is something cseq creating functions uphold
        if let Some(soft_dt) = cseq_part.data.soft_dt
            && soft_dt.reason.reload
        {
            let spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(inv_proj, cseq_part_data_conv.str_mult, spool);
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            // Record only active duration before reload, ignore soft downtime duration
            accum.time += cseq_part.data.active.duration;
            return Some(accum);
        }
        let part_cycle_count = match cseq_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            // If any cycle repeats infinitely without running out, then it does not run out of
            // "clip", no clip - no data
            InfCount::Infinite => return None,
        };
        for i in Count::ZERO..part_cycle_count {
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cseq_part_data_conv.soft_dt {
                // Current cycle is at 0 spool, and we have an interrupt every cycle
                true if uninterrupted_cycles == Count::ZERO => Some(Value::ZERO),
                // Current cycle is at max spool, and we have no interrupts in cycles of current
                // part
                false if uninterrupted_cycles >= inv_spool.cycles_to_max => Some(inv_spool.max),
                _ => None,
            };
            if let Some(stable_spool) = stable_spool {
                let remaining_cycles = part_cycle_count - i;
                let cycle_output = get_proj_spool_cycle_output(inv_proj, cseq_part_data_conv.str_mult, stable_spool);
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, remaining_cycles);
                accum.time += cseq_part_data_conv.cycle_main_duration * remaining_cycles.into_pvalue();
                if !cseq_part_data_conv.soft_dt {
                    uninterrupted_cycles += remaining_cycles;
                }
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            // Case when cycle is at zero spool and will stay at zero spool for the rest of the part
            let spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(inv_proj, cseq_part_data_conv.str_mult, spool);
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            accum.time += cseq_part_data_conv.cycle_main_duration;
            match cseq_part_data_conv.soft_dt {
                true => uninterrupted_cycles = Count::ZERO,
                false => uninterrupted_cycles += Count::ONE,
            }
        }
    }
    match cseq_parts.loops {
        // If we went through all parts without reloads, and they loop, then there is no "clip"
        true => None,
        false => Some(accum),
    }
}

fn aclip_process_both_for_cseq_spool_hard_dt<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    mut accum: SeqAccum<IA>,
    mut converter: C,
) -> Option<SeqAccum<IA>>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>
        + LibConverter<CycleDataFull, AggrPartDataTail<I>>
        + LibConverter<CycleDataFull, AggrPartDataSpoolTail>,
{
    match cseq {
        // Infinite cycle with hard DT never spools up, process it the non-spool way
        CycleSeq::LoopSin(_) => aclip_process_both_for_cseq_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
        CycleSeq::LoopLimSin(inner) => match inner.p1_data.soft_dt {
            // Composite loop with soft downtimes in first part and hard downtime after second also
            // does not spool up
            Some(_) => aclip_process_both_for_cseq_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
            None => {
                // Case when all sequence cycles are allowed to run, possibly with reload after the
                // last cycle
                let inner_conv = inner.convert_with(&mut converter);
                process_output_for_lls_cseq_spool_hard_dt(&inner_conv, inv_proj, inv_spool, &mut accum.instances);
                // Record time until reload or hard downtime starts
                let p2_final_cycle_duration = match inner.p2_data.soft_dt {
                    Some(soft_dt) if soft_dt.reason.reload => inner.p2_data.active.duration,
                    _ => inner_conv.p2_data.cycle_main_duration,
                };
                accum.time += inner_conv
                    .p1_data
                    .cycle_main_duration
                    .mul_add(inner_conv.p1_repeat_count.into_pvalue(), p2_final_cycle_duration);
                Some(accum)
            }
        },
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}
