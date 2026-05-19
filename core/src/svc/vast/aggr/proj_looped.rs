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
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
};

// Projected effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_looped<BG, BX, I, IA>(
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
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(cseq) = cseq.try_loop_cseq() else {
        return false;
    };
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
    true
}

fn process_regular<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    accum: &mut SeqAccum<IA>,
) where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    for cycle_part in cseq.iter_cseq_parts() {
        if cycle_part.repeat_count == Count::ZERO {
            continue;
        }
        let cycle_output = get_proj_regular_output(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        accum.add_output_full(&cycle_output, inv_proj.chance_mult, cycle_part.repeat_count);
        accum.time += cycle_part.data.get_main_duration() * cycle_part.repeat_count.into_pvalue();
    }
}

fn process_hard_dt<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    accum: &mut SeqAccum<IA>,
) where
    BG: NEffectOutputGetter,
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
    let cseq_conv: CycleSeqLooped<_, CSeqHardDtFull> = cseq.convert_with_and_optimize(&mut converter);
    match cseq_conv {
        CycleSeqLooped::Inf(inner) => {
            process_output_of_cycle_with_cutoff(&mut accum.instances, &inner.data, inv_proj.chance_mult, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
        CycleSeqLooped::LoopLimSin(inner) => {
            process_output_of_lls_with_cutoff(&mut accum.instances, &inner, inv_proj.chance_mult, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
    }
}

fn process_spool<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
) where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Part-specific strength mult
        let part_str_mult = get_proj_spool_part_str_mult(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        for i in Count::ZERO..cycle_part.repeat_count {
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cycle_part.data.soft_dt {
                // Current cycle is at 0 spool, and we have an interrupt every cycle
                Some(_) if uninterrupted_cycles == Count::ZERO => Some(Value::ZERO),
                // Current cycle is at max spool, and we have no interrupts in cycles of current
                // part
                None if uninterrupted_cycles >= inv_spool.cycles_to_max => {
                    let remaining_cycles = cycle_part.repeat_count - i;
                    uninterrupted_cycles += remaining_cycles;
                    Some(inv_spool.max)
                }
                _ => None,
            };
            if let Some(stable_spool) = stable_spool {
                let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, stable_spool);
                let remaining_cycles = cycle_part.repeat_count - i;
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, remaining_cycles);
                accum.time += cycle_part.data.get_main_duration() * remaining_cycles.into_pvalue();
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, cycle_spool);
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            accum.time += cycle_part.data.get_main_duration();
            // Update state
            match cycle_part.data.soft_dt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
        }
    }
}

fn process_spool_hard_dt<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
) where
    BG: NEffectOutputGetter,
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let cseq = match cseq {
        // Infinite cycle with hard DT never spools up, process it the non-spool way
        CycleSeqLooped::Inf(_) => {
            process_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum);
            return;
        }
        CycleSeqLooped::LoopLimSin(inner) => match inner.p1_data.soft_dt {
            // Composite loop with soft downtimes in first part and hard downtime after second also
            // does not spool up
            Some(_) => {
                process_hard_dt(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum);
                return;
            }
            None => inner,
        },
    };
    let loop_inner_duration = cseq.get_full_duration();
    let loop_full_duration = loop_inner_duration + cseq.hard_dt.unwrap().duration;
    process_output_of_spooling_lls_with_cutoff(
        ctx,
        calc,
        projector_uid,
        &cseq,
        ospec,
        &inv_proj,
        &inv_spool,
        &mut accum.instances,
        loop_inner_duration,
    );
    accum.time += loop_full_duration;
}

fn get_uninterrupted_cycles(
    cseq: &CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    inv_spool: &AggrSpoolInvData,
) -> Count {
    let mut uninterrupted_cycles = Count::ZERO;
    if cseq.get_hard_dt().is_some() {
        return uninterrupted_cycles;
    }
    let mut downtimes = false;
    for cycle_part in cseq.iter_cseq_parts() {
        match cycle_part.data.soft_dt {
            Some(_) => {
                uninterrupted_cycles = Count::ZERO;
                downtimes = true;
            }
            None => {
                uninterrupted_cycles += cycle_part.repeat_count;
            }
        }
    }
    // If there are no interruptions at all, just set max possible spool right away
    if !downtimes {
        uninterrupted_cycles = inv_spool.cycles_to_max;
    }
    uninterrupted_cycles
}
