use super::{
    accum::SeqAccum,
    local_shared::{AggrLocalInvData, get_local_output},
    precalc::aggr_precalc_by_time,
    traits::{InstanceDuration, LimitInstance},
};
use crate::{
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemId,
};

// Local effects, aggregates total output by specified time
pub(in crate::svc::vast) fn aggr_local_time<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
    accum: &mut A,
    time: PValue,
) where
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
    A: SeqAccum<T> + Default,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec) {
        Some(inv_local) => inv_local,
        None => return,
    };
    let precalc = match cseq {
        CycleSeq::Lim(inner) => {
            let opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::Inf(inner) => {
            let opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::LimInf(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
        CycleSeq::LimSinInf(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p2_data.chargedness);
            let p3_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p3_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc, p3_opc)
        }
        CycleSeq::LoopLimSin(inner) => {
            let p1_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p1_data.chargedness);
            let p2_opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
    };
    aggr_precalc_by_time(precalc, None, accum, time);
}
