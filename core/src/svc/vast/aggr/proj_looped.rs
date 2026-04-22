use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, get_proj_regular_output, get_proj_spool_cycle_output,
        get_proj_spool_part_str_mult,
    },
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataDurCharge, CycleDataFull, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
};

// Projected effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_looped<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_proj = match AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid) {
        Some(inv_proj) => inv_proj,
        None => return false,
    };
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum),
        None => aggr_regular(
            ctx,
            calc,
            projector_uid,
            cseq.convert_and_optimize(),
            ospec,
            inv_proj,
            accum,
        ),
    }
}

fn aggr_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: CycleSeq<CycleDataDurCharge>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    for cycle_part in cseq.iter_cseq_parts() {
        if cycle_part.repeat_count == Count::ZERO {
            continue;
        }
        let cycle_output =
            get_proj_regular_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle_part.data.chargedness);
        accum.add_instance(
            cycle_output.get_instance(),
            inv_proj.chance_mult,
            cycle_output.get_instance_count() * cycle_part.repeat_count,
        );
        accum.time += cycle_part.data.active_duration * cycle_part.repeat_count.into_pvalue();
    }
    true
}

fn aggr_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Part-specific strength mult
        let part_str_mult =
            get_proj_spool_part_str_mult(ctx, calc, projector_uid, ospec, &inv_proj, cycle_part.data.chargedness);
        for i in Count::ZERO..cycle_part.repeat_count {
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cycle_part.data.interrupt {
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
                // Update total values
                let remaining_cycles = cycle_part.repeat_count - i;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * remaining_cycles,
                );
                accum.time += cycle_part.data.active_duration * remaining_cycles.into_pvalue();
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, cycle_spool);
            // Update total values
            accum.add_instance(
                cycle_output.get_instance(),
                inv_proj.chance_mult,
                cycle_output.get_instance_count(),
            );
            accum.time += cycle_part.data.active_duration;
            // Update state
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
        }
    }
    true
}

fn get_uninterrupted_cycles(cseq: &CycleSeqLooped<CycleDataFull>, inv_spool: &AggrSpoolInvData) -> Count {
    let mut uninterrupted_cycles = Count::ZERO;
    let mut interruptions = false;
    for cycle_part in cseq.iter_cseq_parts() {
        match cycle_part.data.interrupt {
            Some(_) => {
                uninterrupted_cycles = Count::ZERO;
                interruptions = true;
            }
            None => {
                uninterrupted_cycles += cycle_part.repeat_count;
            }
        }
    }
    // If there are no interruptions at all, just set max possible spool right away
    if !interruptions {
        uninterrupted_cycles = inv_spool.cycles_to_max;
    }
    uninterrupted_cycles
}
