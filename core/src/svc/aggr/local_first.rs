use super::{
    local_inv_data::LocalInvariantData,
    shared::{AggrAmount, AggrOutput},
    traits::LimitAmount,
};
use crate::{
    AttrVal,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
};

// Local effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_local_first_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + std::ops::Div<AttrVal, Output = T>
        + LimitAmount,
{
    aggr_local_first_amount(ctx, calc, item_key, effect, cseq, ospec).and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_local_first_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrAmount<T>>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T> + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    aggr_local_first_output(ctx, calc, item_key, effect, cseq, ospec).map(|output_data| AggrAmount {
        amount: output_data.output.amount_sum(),
        time: output_data.time,
    })
}

pub(in crate::svc) fn aggr_local_first_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrOutput<T>>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let cycle = cseq.get_first_cycle();
    let inv_local = LocalInvariantData::try_make(ctx, calc, item_key, effect, ospec)?;
    let mut output = inv_local.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = cycle.chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
    {
        output *= charge_mult;
    }
    // Limit
    if let Some(limit) = inv_local.amount_limit {
        output.limit_amount(limit);
    }
    Some(AggrOutput {
        output,
        time: cycle.time,
    })
}
