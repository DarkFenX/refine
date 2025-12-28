use super::{
    proj_inv_data::{ProjInvariantData, SpoolInvariantData},
    shared::AggrData,
    traits::{LimitAmount, Maximum},
};
use crate::{
    def::{AttrVal, Count, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleDataTimeCharge, CycleSeq, CycleSeqLooped},
        output::Output,
    },
    ud::UItemKey,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_proj_looped_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + std::ops::Div<AttrVal, Output = T>
        + LimitAmount,
{
    aggr_proj_looped_data(ctx, calc, projector_key, effect, cseq, ospec, projectee_key)
        .and_then(|aggr_data| aggr_data.get_ps())
}

pub(in crate::svc) fn aggr_proj_looped_max<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount
        + Maximum,
{
    match SpoolInvariantData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_max_spool(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, inv_spool),
        None => aggr_max_regular(ctx, calc, projector_key, effect, cseq.into(), ospec, projectee_key),
    }
}

pub(in crate::svc) fn aggr_proj_looped_data<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<AggrData<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    match SpoolInvariantData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_total_spool(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, inv_spool),
        None => aggr_total_regular(ctx, calc, projector_key, effect, cseq.into(), ospec, projectee_key),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Totals
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_total_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeCharge>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<AggrData<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let mut part_output = inv_proj.output;
        // Chargedness
        if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
        {
            part_output *= charge_mult;
        }
        // Limit
        if let Some(limit) = inv_proj.amount_limit {
            part_output.limit_amount(limit);
        }
        // Chance-based multipliers
        if let Some(mult_post) = inv_proj.mult_post {
            part_output *= mult_post;
        }
        // Update total values
        let part_cycle_count = AttrVal::from(cycle_part.repeat_count);
        total_amount += part_output.amount_sum() * part_cycle_count;
        total_time += cycle_part.data.time * part_cycle_count;
    }
    Some(AggrData {
        amount: total_amount,
        time: total_time,
    })
}

fn aggr_total_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    inv_spool: SpoolInvariantData,
) -> Option<AggrData<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
        {
            charge_mult_getter(ctx, calc, projector_key, chargedness)
        } else {
            None
        };
        for i in 0..cycle_part.repeat_count {
            let cycle_output = inv_proj.output;
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cycle_part.data.interrupt {
                // Current cycle is at 0 spool, and we have an interrupt every cycle
                Some(_) if uninterrupted_cycles == 0 => Some(OF(0.0)),
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
                let cycle_output = apply_values_spool(
                    cycle_output,
                    charge_mult,
                    stable_spool,
                    inv_proj.amount_limit,
                    inv_proj.mult_post,
                );
                // Update total values
                let remaining_cycles = AttrVal::from(cycle_part.repeat_count - i);
                total_amount += cycle_output.amount_sum() * remaining_cycles;
                total_time += cycle_part.data.time * remaining_cycles;
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_output = apply_values_spool(
                cycle_output,
                charge_mult,
                inv_spool.max.min(inv_spool.step * uninterrupted_cycles as f64),
                inv_proj.amount_limit,
                inv_proj.mult_post,
            );
            // Update total values
            total_amount += cycle_output.amount_sum();
            total_time += cycle_part.data.time;
            // Update state
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = 0,
                None => uninterrupted_cycles += 1,
            }
        }
    }
    Some(AggrData {
        amount: total_amount,
        time: total_time,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Max
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_max_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeCharge>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount
        + Maximum,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut max_amount = T::default();
    for cycle_part in cseq.iter_cseq_parts() {
        let mut part_output = inv_proj.output;
        // Chargedness
        if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
        {
            part_output *= charge_mult;
        }
        // Limit
        if let Some(limit) = inv_proj.amount_limit {
            part_output.limit_amount(limit);
        }
        // Chance-based multipliers
        if let Some(mult_post) = inv_proj.mult_post {
            part_output *= mult_post;
        }
        // Update result
        max_amount = max_amount.maximum(part_output.get_max_amount());
    }
    Some(max_amount)
}

fn aggr_max_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    inv_spool: SpoolInvariantData,
) -> Option<T>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount
        + Maximum,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    let mut max_amount = T::default();
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
        {
            charge_mult_getter(ctx, calc, projector_key, chargedness)
        } else {
            None
        };
        for i in 0..cycle_part.repeat_count {
            let cycle_output = inv_proj.output;
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cycle_part.data.interrupt {
                // Current cycle is at 0 spool, and we have an interrupt every cycle
                Some(_) if uninterrupted_cycles == 0 => Some(OF(0.0)),
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
                let cycle_output = apply_values_spool(
                    cycle_output,
                    charge_mult,
                    stable_spool,
                    inv_proj.amount_limit,
                    inv_proj.mult_post,
                );
                // Update result
                max_amount = max_amount.maximum(cycle_output.get_max_amount());
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_output = apply_values_spool(
                cycle_output,
                charge_mult,
                inv_spool.max.min(inv_spool.step * uninterrupted_cycles as f64),
                inv_proj.amount_limit,
                inv_proj.mult_post,
            );
            // Update result
            max_amount = max_amount.maximum(cycle_output.get_max_amount());
            // Update state
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = 0,
                None => uninterrupted_cycles += 1,
            }
        }
    }
    Some(max_amount)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_values_spool<T>(
    mut output: Output<T>,
    chargedness: Option<AttrVal>,
    spool: AttrVal,
    amount_limit: Option<AttrVal>,
    mult_post: Option<AttrVal>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    if let Some(charge_mult) = chargedness {
        output *= charge_mult;
    }
    output *= OF(1.0) + spool;
    if let Some(limit) = amount_limit {
        output.limit_amount(limit);
    }
    if let Some(mult_post) = mult_post {
        output *= mult_post;
    }
    output
}

fn get_uninterrupted_cycles(cseq: &CycleSeqLooped<CycleDataFull>, inv_spool: &SpoolInvariantData) -> Count {
    let mut uninterrupted_cycles = 0;
    let mut interruptions = false;
    for cycle_part in cseq.iter_cseq_parts() {
        match cycle_part.data.interrupt {
            Some(_) => {
                uninterrupted_cycles = 0;
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
