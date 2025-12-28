use super::{local_inv_data::LocalInvariantData, shared::AggrAmount, traits::LimitAmount};
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
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
    aggr_local_looped_amount(ctx, calc, item_key, effect, cseq, ospec).and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_local_looped_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_local = LocalInvariantData::try_make(ctx, calc, item_key, effect, ospec)?;
    let mut total_amount = T::default();
    let mut total_time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
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
        let part_cycle_count = AttrVal::from(cycle_part.repeat_count);
        total_amount += part_output.amount_sum() * part_cycle_count;
        total_time += cycle_part.data.time * part_cycle_count;
    }
    Some(AggrAmount {
        amount: total_amount,
        time: total_time,
    })
}
