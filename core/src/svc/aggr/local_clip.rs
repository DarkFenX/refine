use super::{local_inv_data::LocalInvariantData, shared::AggrData, traits::LimitAmount};
use crate::{
    AttrVal,
    def::OF,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
    util::InfCount,
};

// Local effects, considers only part of sequence until charges are out
pub(in crate::svc) fn aggr_local_clip_data<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrData<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let inv_local = LocalInvariantData::try_make(ctx, calc, item_key, effect, ospec)?;
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let mut part_output = inv_local.output;
        // Chargedness
        if let Some(charge_mult_getter) = ospec.charge_mult
            && let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            part_output *= charge_mult;
        }
        // Limit
        if let Some(limit) = inv_local.amount_limit {
            part_output.limit_amount(limit);
        }
        // Update total values
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                total_amount += part_output.amount_sum();
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
                total_amount += part_output.amount_sum() * part_cycle_count;
                total_time += cycle_part.data.time * part_cycle_count;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    if cycle_parts.loops && !reload {
        return None;
    }
    Some(AggrData {
        amount: total_amount,
        time: total_time,
    })
}
