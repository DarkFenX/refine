use super::{
    local_shared::{AggrLocalInvData, get_local_output},
    precalc::aggr_precalc_by_time,
    traits::LimitAmount,
};
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemId,
};

// Local effects, aggregates total output by specified time
pub(in crate::svc) fn aggr_local_time_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
    time: AttrVal,
) -> Option<T>
where
    T: Default
        + Copy
        + Eq
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + std::ops::Div<AttrVal, Output = T>
        + LimitAmount,
{
    aggr_local_time_amount(ctx, calc, item_key, effect, cseq, ospec, time).map(|v| v / time)
}

pub(in crate::svc) fn aggr_local_time_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
    time: AttrVal,
) -> Option<T>
where
    T: Default
        + Copy
        + Eq
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    if time < OF(0.0) {
        return None;
    }
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_key, effect, ospec)?;
    let precalc = match cseq {
        CycleSeq::Lim(inner) => {
            let opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::Inf(inner) => {
            let opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::LimInf(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
        CycleSeq::LimSinInf(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p2_data.chargedness);
            let p3_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p3_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc, p3_opc)
        }
        CycleSeq::LoopLimSin(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
    };
    Some(aggr_precalc_by_time(precalc, time))
}
