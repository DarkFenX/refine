use super::{
    precalc::aggr_precalc_by_time,
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_output},
    traits::LimitAmount,
};
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
};

// Projected effects, aggregates total output by specified time
pub(in crate::svc) fn aggr_proj_time_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
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
    match AggrSpoolInvData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_total_regular(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, time),
        None => aggr_total_regular(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, time),
    }
}

fn aggr_total_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
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
    let inv_local = AggrProjInvData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let precalc = match cseq {
        CycleSeq::Lim(inner) => {
            let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::Inf(inner) => {
            let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::LimInf(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
        CycleSeq::LimSinInf(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p2_data.chargedness);
            let p3_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p3_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc, p3_opc)
        }
        CycleSeq::LoopLimSin(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
    };
    Some(aggr_precalc_by_time(precalc, time))
}
