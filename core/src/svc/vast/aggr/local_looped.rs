use super::{
    local_shared::{AggrLocalInvData, get_local_output},
    shared::AggrAmount,
    traits::LimitAmount,
};
use crate::{
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemId,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
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
    aggr_local_looped_amount(ctx, calc, item_uid, effect, cseq, ospec).and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_local_looped_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
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
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec)?;
    let mut total_amount = T::default();
    let mut total_time = PValue::ZERO;
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_part.data.chargedness);
        let part_cycle_count = cycle_part.repeat_count.into_pvalue();
        total_amount += cycle_output.get_amount_sum() * part_cycle_count;
        total_time += cycle_part.data.duration * part_cycle_count;
    }
    Some(AggrAmount {
        amount: total_amount,
        duration: total_time,
    })
}
