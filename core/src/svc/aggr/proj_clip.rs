use super::{
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_output, get_proj_output_spool},
    shared::{AggrAmount, calc_charge_mult},
    traits::LimitAmount,
};
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::InfCount,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_proj_clip_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemId>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    match AggrSpoolInvData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_spool(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, inv_spool),
        None => aggr_regular(ctx, calc, projector_key, effect, cseq, ospec, projectee_key),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemId>,
    inv_spool: AggrSpoolInvData,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut uninterrupted_cycles = 0;
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
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
        let charge_mult = calc_charge_mult(ctx, calc, projector_key, ospec.charge_mult, cycle_part.data.chargedness);
        for i in 0..part_cycle_count {
            // Case when the rest of cycle part is at full spool
            if cycle_part.data.interrupt.is_none() && uninterrupted_cycles >= inv_spool.cycles_to_max {
                let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, inv_spool.max);
                let remaining_cycles = part_cycle_count - i;
                uninterrupted_cycles += remaining_cycles;
                let remaining_cycles = AttrVal::from(remaining_cycles);
                total_amount += cycle_output.get_amount_sum() * remaining_cycles;
                total_time += cycle_part.data.time * remaining_cycles;
                // No interruptions in this branch, no need to do handle reload flag
                continue 'part;
            }
            let spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, spool);
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = 0,
                None => uninterrupted_cycles += 1,
            }
            total_amount += cycle_output.get_amount_sum();
            total_time += cycle_part.data.time;
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
    Some(AggrAmount {
        amount: total_amount,
        time: total_time,
    })
}

fn aggr_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemId>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, cycle_part.data.chargedness);
        // Update total values
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                total_amount += cycle_output.get_amount_sum();
                total_time += cycle_part.data.time;
                break;
            }
            _ => {
                let part_cycle_count = match cycle_part.repeat_count {
                    InfCount::Count(part_cycle_count) => AttrVal::from(part_cycle_count),
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return None,
                };
                total_amount += cycle_output.get_amount_sum() * part_cycle_count;
                total_time += cycle_part.data.time * part_cycle_count;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    if cycle_parts.loops && !reload {
        return None;
    }
    Some(AggrAmount {
        amount: total_amount,
        time: total_time,
    })
}
