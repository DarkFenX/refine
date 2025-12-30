use super::{
    local_shared::{AggrLocalInvData, get_local_output},
    traits::LimitAmount,
};
use crate::{
    def::{AttrVal, Count, OF},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemKey,
    util::trunc_unerr,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_time_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
    mut time: AttrVal,
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
    let cache = match cseq {
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
    let mut total_amount = T::default();
    match cache {
        CycleSeq::Lim(inner) => {
            let full_repeats = (get_count_full_repeats(time, inner.data.time, inner.data.tail_time).trunc() as Count)
                .min(inner.repeat_count);
            total_amount += inner.data.output.get_amount_sum() * AttrVal::from(full_repeats);
            let mut remaining_repeats = inner.repeat_count - full_repeats;
            while time >= OF(0.0) && remaining_repeats > 0 {
                total_amount += inner.data.output.get_amount_sum_by_time(time);
                time -= inner.data.time;
                remaining_repeats -= 1;
            }
        }
        CycleSeq::Inf(inner) => {
            let full_repeats = get_count_full_repeats(time, inner.data.time, inner.data.tail_time);
            total_amount += inner.data.output.get_amount_sum() * full_repeats;
            time -= inner.data.time * full_repeats;
            while time >= OF(0.0) {
                total_amount += inner.data.output.get_amount_sum_by_time(time);
                time -= inner.data.time;
            }
        }
        CycleSeq::LimInf(inner) => {}
        CycleSeq::LimSinInf(inner) => {}
        CycleSeq::LoopLimSin(inner) => {}
    }
    Some(total_amount)
}

fn get_count_full_repeats(time: AttrVal, cycle_time: AttrVal, cycle_tail_time: AttrVal) -> AttrVal {
    let time_no_tail = time - cycle_tail_time;
    if time_no_tail < cycle_time {
        return OF(0.0);
    }
    trunc_unerr(time_no_tail / cycle_time)
}
