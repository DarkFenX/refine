use super::{local_inv_data::LocalInvariantData, shared::AggrAmountData, traits::Aggregable};
use crate::{
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
};

// Local effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_local_first_amount_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    Some(aggr_local_first_amount_data(ctx, calc, item_key, effect, cseq, ospec)?.get_ps()?)
}

pub(in crate::svc) fn aggr_local_first_amount_data<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrAmountData<T>>
where
    T: Copy + Aggregable,
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
    Some(AggrAmountData {
        amount: output.instance_sum(),
        time: cycle.time,
    })
}
