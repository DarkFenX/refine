use super::{
    local_shared::{AggrLocalInvData, get_local_output},
    shared::{AggrAmount, AggrOutput},
    traits::LimitAmount,
};
use crate::{
    misc::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemId,
};

// Local effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_local_first_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
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
    aggr_local_first_amount(ctx, calc, item_uid, effect, cseq, ospec).and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_local_first_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrAmount<T>>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T> + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    aggr_local_first_output(ctx, calc, item_uid, effect, cseq, ospec).map(|output_data| AggrAmount {
        amount: output_data.output.get_amount_sum(),
        time: output_data.time,
    })
}

pub(in crate::svc) fn aggr_local_first_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrOutput<T>>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let cycle_data = cseq.get_first_cycle();
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec)?;
    Some(AggrOutput {
        output: get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_data.chargedness),
        time: cycle_data.time,
    })
}
