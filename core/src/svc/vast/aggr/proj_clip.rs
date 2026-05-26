use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverter, get_proj_spool_cycle_output, get_proj_spool_part_str_mult,
        process_output_of_spooling_lls_with_cutoff,
    },
    shared::{AggrPartData, AggrPartDataSpoolTail, AggrPartDataTail},
    shared_clip::{process_hard_dt, process_regular},
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
    let converter = ProjConverter::new(ctx, calc, projector_uid, ospec, &inv_proj);
    match (inv_spool, cseq.get_hard_dt().is_some()) {
        (Some(inv_spool), true) => process_spool_hard_dt(cseq, &inv_proj, &inv_spool, accum, converter),
        (Some(inv_spool), false) => process_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum),
        (None, true) => process_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
        (None, false) => process_regular(cseq, inv_proj.chance_mult, accum, converter),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
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
    let cseq_parts = cseq.get_cseq_parts();
    'part: for cseq_part in cseq_parts.iter() {
        let part_cycle_count = match cseq_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            InfCount::Infinite => match cseq_part.data.soft_dt {
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
            cseq_part.data.active.chargedness,
        );
        let part_cycle_main_duration = cseq_part.data.get_main_duration();
        for i in Count::ZERO..part_cycle_count {
            // Shortcut #1: we're at 0 spool and can't spool for the rest of the sequence
            if let Some(soft_dt) = cseq_part.data.soft_dt
                && uninterrupted_cycles == Count::ZERO
            {
                let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, Value::ZERO);
                // For a cycle followed by a reload, consider clip finished - add just it (with only
                // pre-reload duration recorded), set reload flag and quit processing
                if soft_dt.reason.reload {
                    accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
                    accum.time += cseq_part.data.active.duration;
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
            if cseq_part.data.soft_dt.is_none() && uninterrupted_cycles >= inv_spool.cycles_to_max {
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
            match cseq_part.data.soft_dt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            // For a cycle followed by a reload, consider clip finished - add just it (with only
            // pre-reload duration recorded), set reload flag and quit processing
            if let Some(soft_dt) = cseq_part.data.soft_dt
                && soft_dt.reason.reload
            {
                accum.time += cseq_part.data.active.duration;
                reload = true;
                break 'part;
            }
            accum.time += part_cycle_main_duration;
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cseq_parts.loops || reload
}

fn process_spool_hard_dt<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
    mut converter: C,
) -> bool
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>
        + LibConverter<CycleDataFull, AggrPartDataTail<I>>
        + LibConverter<CycleDataFull, AggrPartDataSpoolTail>,
{
    match cseq {
        // Infinite cycle with hard DT never spools up, process it the non-spool way
        CycleSeq::Inf(_) => process_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
        CycleSeq::LoopLimSin(inner) => match inner.p1_data.soft_dt {
            // Composite loop with soft downtimes in first part and hard downtime after second also
            // does not spool up
            Some(_) => process_hard_dt(cseq, inv_proj.chance_mult, accum, converter),
            None => {
                let inner_conv = inner.convert_with(&mut converter);
                // No soft downtime in first part in this case, the only variance is having soft
                // downtime in the second part
                let loop_inner_duration = inner.get_full_duration();
                process_output_of_spooling_lls_with_cutoff(
                    &inner_conv,
                    inv_proj,
                    inv_spool,
                    &mut accum.instances,
                    loop_inner_duration,
                );
                // Record time until reload or hard downtime starts
                match inner.p2_data.soft_dt {
                    Some(soft_dt) if soft_dt.reason.reload => {
                        accum.time += inner.get_full_duration_without_p2_soft_dt()
                    }
                    _ => accum.time += loop_inner_duration,
                }
                true
            }
        },
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}
