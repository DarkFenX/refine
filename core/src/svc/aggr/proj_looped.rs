use super::{
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_output, get_proj_output_spool},
    shared::{AggrAmount, calc_charge_mult},
    traits::LimitAmount,
};
use crate::{
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataDurCharge, CycleDataFull, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
    util::LibMax,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_proj_looped_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + std::ops::Div<PValue, Output = T>
        + LimitAmount,
{
    aggr_proj_looped_amount(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid)
        .and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_proj_looped_max<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LibMax
        + LimitAmount,
{
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_max_spool(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid, inv_spool),
        None => aggr_max_regular(ctx, calc, projector_uid, effect, cseq.convert(), ospec, projectee_uid),
    }
}

pub(in crate::svc) fn aggr_proj_looped_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LimitAmount,
{
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_total_spool(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid, inv_spool),
        None => aggr_total_regular(ctx, calc, projector_uid, effect, cseq.convert(), ospec, projectee_uid),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Totals
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_total_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: CycleSeq<CycleDataDurCharge>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    let mut total_amount = T::default();
    let mut total_time = PValue::ZERO;
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_output = get_proj_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle_part.data.chargedness);
        let part_cycle_count = cycle_part.repeat_count.into_pvalue();
        total_amount += cycle_output.get_amount_sum() * part_cycle_count;
        total_time += cycle_part.data.duration * part_cycle_count;
    }
    Some(AggrAmount {
        amount: total_amount,
        duration: total_time,
    })
}

fn aggr_total_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    inv_spool: AggrSpoolInvData,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    let mut total_amount = T::default();
    let mut total_time = PValue::ZERO;
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = calc_charge_mult(ctx, calc, projector_uid, ospec.charge_mult, cycle_part.data.chargedness);
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
                let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, stable_spool);
                // Update total values
                let remaining_cycles = (cycle_part.repeat_count - i).into_pvalue();
                total_amount += cycle_output.get_amount_sum() * remaining_cycles;
                total_time += cycle_part.data.duration * remaining_cycles;
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, cycle_spool);
            // Update total values
            total_amount += cycle_output.get_amount_sum();
            total_time += cycle_part.data.duration;
            // Update state
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
        }
    }
    Some(AggrAmount {
        amount: total_amount,
        duration: total_time,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Max
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_max_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: CycleSeq<CycleDataDurCharge>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LibMax
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    let mut max_amount = T::default();
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_output = get_proj_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle_part.data.chargedness);
        // Update result
        max_amount = max_amount.lib_max(cycle_output.get_max_amount());
    }
    Some(max_amount)
}

fn aggr_max_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    inv_spool: AggrSpoolInvData,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LibMax
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    let mut max_amount = T::default();
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = calc_charge_mult(ctx, calc, projector_uid, ospec.charge_mult, cycle_part.data.chargedness);
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
                let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, stable_spool);
                max_amount = max_amount.lib_max(cycle_output.get_max_amount());
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, cycle_spool);
            // Update result
            max_amount = max_amount.lib_max(cycle_output.get_max_amount());
            // Update state
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
        }
    }
    Some(max_amount)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
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
