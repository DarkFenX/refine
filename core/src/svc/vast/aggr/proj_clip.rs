use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverterRegular, get_proj_regular_output, get_proj_spool_cycle_output,
        get_proj_spool_part_str_mult, process_output_of_spooling_lls_with_cutoff,
    },
    shared::{process_output_of_cycle_with_cutoff, process_output_of_lls_with_cutoff},
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
        cycle::{CSeqHardDtFull, CSeqInf, CSeqLoopLimSin, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Projected effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_clip<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_proj) = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)
    else {
        return false;
    };
    let inv_spool = AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec);
    match (inv_spool, cseq.get_hard_dt().is_some()) {
        (Some(inv_spool), true) => {
            process_spool_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum)
        }
        (Some(inv_spool), false) => process_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum),
        (None, true) => process_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
        (None, false) => process_regular(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_regular<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_proj_regular_output(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        match cycle_part.data.soft_dt {
            // Add first cycle after which there is a reload
            Some(soft_dt) if soft_dt.reason.reload => {
                reload = true;
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
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
                    accum.add_output_full(&cycle_output, inv_proj.chance_mult, part_cycle_count);
                    accum.time += cycle_part.data.get_main_duration() * part_cycle_count.into_pvalue();
                }
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}

fn process_hard_dt<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        // Infinite cycle with hard downtime on every cycle means we have just that cycle in clip
        CycleSeq::Inf(inner) => {
            let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
            let inner_conv: CSeqInf<_, CSeqHardDtFull> = inner.convert_with(&mut converter);
            process_output_of_cycle_with_cutoff(
                &mut accum.instances,
                &inner_conv.data,
                inv_proj.chance_mult,
                Count::ONE,
            );
            // Record time until reload or hard downtime starts
            match inner.data.soft_dt {
                Some(soft_dt) if soft_dt.reason.reload => accum.time += inner.data.active.duration,
                _ => accum.time += inner_conv.data.cycle_main_duration,
            }
            true
        }
        CycleSeq::LoopLimSin(inner) => {
            if let Some(soft_dt) = inner.p1_data.soft_dt
                && soft_dt.reason.reload
            {
                // Case when there is a reload right after first cycle
                let output = get_proj_regular_output(
                    ctx,
                    calc,
                    projector_uid,
                    ospec,
                    &inv_proj,
                    inner.p1_data.active.chargedness,
                );
                let loop_inner_duration = inner.get_full_duration();
                match inv_proj.get_output_completion_duration() > loop_inner_duration {
                    true => accum.add_output_time_limited(
                        &output,
                        inv_proj.chance_mult,
                        Count::ONE,
                        loop_inner_duration.into_value(),
                    ),
                    false => accum.add_output_full(&output, inv_proj.chance_mult, Count::ONE),
                }
                // Stop counting time at reload, after active cycle is finished
                accum.time += inner.p1_data.active.duration;
            } else {
                // Case when all sequence cycles are allowed to run, possibly with reload after the
                // last cycle
                let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                let inner_conv: CSeqLoopLimSin<_, CSeqHardDtFull> = inner.convert_with(&mut converter);
                process_output_of_lls_with_cutoff(&mut accum.instances, &inner_conv, inv_proj.chance_mult, Count::ONE);
                // Record time until reload or hard downtime starts
                match inner.p2_data.soft_dt {
                    Some(soft_dt) if soft_dt.reason.reload => {
                        accum.time += inner.get_full_duration_without_p2_soft_dt()
                    }
                    _ => accum.time += inner.get_full_duration(),
                }
            }
            true
        }
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}

fn process_spool<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut uninterrupted_cycles = Count::ZERO;
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    'part: for cycle_part in cycle_parts.iter() {
        let part_cycle_count = match cycle_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            InfCount::Infinite => match cycle_part.data.soft_dt {
                // Process 1 cycle if reload happens after every cycle in this part, even if cycles
                // are infinite
                Some(soft_dt) if soft_dt.reason.reload => Count::ONE,
                // No reloads in infinite sequence - sequence is not a clip - no data to return
                _ => return false,
            },
        };
        // Part-specific strength mult
        let part_str_mult = get_proj_spool_part_str_mult(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        let part_cycle_main_duration = cycle_part.data.get_main_duration();
        for i in Count::ZERO..part_cycle_count {
            // Shortcut #1: we're at 0 spool and can't spool for the rest of the sequence
            if let Some(soft_dt) = cycle_part.data.soft_dt
                && uninterrupted_cycles == Count::ZERO
            {
                let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, Value::ZERO);
                // For a cycle followed by a reload, consider clip finished - add just it (with only
                // pre-reload duration recorded), set reload flag and quit processing
                if soft_dt.reason.reload {
                    accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
                    accum.time += cycle_part.data.active.duration;
                    reload = true;
                    break 'part;
                }
                let remaining_cycles = part_cycle_count - i;
                if remaining_cycles > Count::ZERO {
                    accum.add_output_full(&cycle_output, inv_proj.chance_mult, remaining_cycles);
                    accum.time += part_cycle_main_duration * remaining_cycles.into_pvalue();
                }
                // No interruptions in this branch, no need to handle reload flag and break
                continue 'part;
            }
            // Shortcut #2: we're at max spool and sequence is not interruptable
            if cycle_part.data.soft_dt.is_none() && uninterrupted_cycles >= inv_spool.cycles_to_max {
                let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, inv_spool.max);
                let remaining_cycles = part_cycle_count - i;
                if remaining_cycles > Count::ZERO {
                    uninterrupted_cycles += remaining_cycles;
                    accum.add_output_full(&cycle_output, inv_proj.chance_mult, remaining_cycles);
                    accum.time += part_cycle_main_duration * remaining_cycles.into_pvalue();
                }
                // No interruptions in this branch, no need to handle reload flag and break
                continue 'part;
            }
            // Case when cycle is at zero spool and will stay at zero spool for the rest of the part
            let spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, spool);
            match cycle_part.data.soft_dt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            // For a cycle followed by a reload, consider clip finished - add just it (with only
            // pre-reload duration recorded), set reload flag and quit processing
            if let Some(soft_dt) = cycle_part.data.soft_dt
                && soft_dt.reason.reload
            {
                accum.time += cycle_part.data.active.duration;
                reload = true;
                break 'part;
            }
            accum.time += part_cycle_main_duration;
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}

fn process_spool_hard_dt<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let cseq = match cseq {
        // Infinite cycle with hard DT never spools up, process it the non-spool way
        CycleSeq::Inf(_) => return process_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
        CycleSeq::LoopLimSin(inner) => match inner.p1_data.soft_dt {
            // Composite loop with soft downtimes in first part and hard downtime after second also
            // does not spool up
            Some(_) => return process_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
            None => inner,
        },
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    };
    // No soft downtime in first part in this case, the only variance is having soft downtime in the
    // second part
    let loop_inner_duration = cseq.get_full_duration();
    process_output_of_spooling_lls_with_cutoff(
        ctx,
        calc,
        projector_uid,
        cseq,
        ospec,
        &inv_proj,
        &inv_spool,
        &mut accum.instances,
        loop_inner_duration,
    );
    // Record time until reload or hard downtime starts
    match cseq.p2_data.soft_dt {
        Some(soft_dt) if soft_dt.reason.reload => accum.time += cseq.get_full_duration_without_p2_soft_dt(),
        _ => accum.time += loop_inner_duration,
    }
    true
}
