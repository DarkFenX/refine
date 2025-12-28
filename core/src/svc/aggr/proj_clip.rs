use super::{
    proj_inv_data::{ProjInvariantData, SpoolInvariantData},
    shared::AggrData,
    traits::Aggregable,
};
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemKey,
    util::InfCount,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_proj_clip_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    Some(aggr_proj_clip(ctx, calc, projector_key, effect, cseq, ospec, projectee_key)?.get_per_second())
}

pub(in crate::svc) fn aggr_proj_clip<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<AggrData<T>>
where
    T: Copy + Aggregable,
{
    match SpoolInvariantData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_proj_spool(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, inv_spool),
        None => aggr_proj_regular(ctx, calc, projector_key, effect, cseq, ospec, projectee_key),
    }
}

fn aggr_proj_spool<T>(
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
    T: Copy + Aggregable,
{
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut uninterrupted_cycles = 0;
    let mut value = T::default();
    let mut time = OF(0.0);
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    'part: for cycle_part in cycle_parts.iter() {
        let part_cycle_count = match cycle_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            InfCount::Infinite => match cycle_part.data.interrupt {
                // Process 1 cycle if reload happens after every cycle in this part, even if cycles
                // are infinite
                Some(interrupt) if interrupt.reload => 1,
                // No reloads in infinite sequence - sequence is not a clip - no data to return
                _ => return None,
            },
        };
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
        {
            charge_mult_getter(ctx, calc, projector_key, chargedness)
        } else {
            None
        };
        for i in 0..part_cycle_count {
            let mut part_output = inv_proj.output;
            // Case when the rest of cycle part is at full spool
            if cycle_part.data.interrupt.is_none() && uninterrupted_cycles >= inv_spool.cycles_to_max {
                let remaining_cycles = part_cycle_count - i;
                // Chargedness
                if let Some(charge_mult) = charge_mult {
                    part_output *= charge_mult;
                }
                // Spool
                part_output *= OF(1.0) + inv_spool.max;
                uninterrupted_cycles += remaining_cycles;
                // Limit
                if let Some(limit) = inv_proj.amount_limit {
                    part_output.limit_amount(limit);
                }
                // Chance-based multipliers
                if let Some(mult_post) = inv_proj.mult_post {
                    part_output *= mult_post;
                }
                // Update total values
                let remaining_cycles = AttrVal::from(remaining_cycles);
                value += part_output.instance_sum() * remaining_cycles;
                time += cycle_part.data.time * remaining_cycles;
                // No interruptions in this branch, no need to do handle reload flag
                continue 'part;
            }
            // Chargedness
            if let Some(charge_mult) = charge_mult {
                part_output *= charge_mult;
            }
            // Spool
            part_output *= OF(1.0) + inv_spool.max.min(inv_spool.step * uninterrupted_cycles as f64);
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = 0,
                None => uninterrupted_cycles += 1,
            }
            // Limit
            if let Some(limit) = inv_proj.amount_limit {
                part_output.limit_amount(limit);
            }
            // Chance-based multipliers
            if let Some(mult_post) = inv_proj.mult_post {
                part_output *= mult_post;
            }
            // Update total values - current cycle is added regardless
            value += part_output.instance_sum();
            time += cycle_part.data.time;
            // If reload happens after it, set reload flag and quit all the cycling - clip is
            // considered finished upon hitting reload
            if let Some(interrupt) = cycle_part.data.interrupt
                && interrupt.reload
            {
                reload = true;
                break 'part;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    if cycle_parts.loops && !reload {
        return None;
    }
    Some(AggrData { amount: value, time })
}

fn aggr_proj_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<AggrData<T>>
where
    T: Copy + Aggregable,
{
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut value = T::default();
    let mut time = OF(0.0);
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
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
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                value += part_output.instance_sum();
                time += cycle_part.data.time;
                break;
            }
            _ => {
                let part_cycle_count = match cycle_part.repeat_count {
                    InfCount::Count(part_cycle_count) => AttrVal::from(part_cycle_count),
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return None,
                };
                value += part_output.instance_sum() * part_cycle_count;
                time += cycle_part.data.time * part_cycle_count;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    if cycle_parts.loops && !reload {
        return None;
    }
    Some(AggrData { amount: value, time })
}
