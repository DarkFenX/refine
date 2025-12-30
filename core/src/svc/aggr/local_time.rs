use super::{
    cache::AggrPartData,
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
            if time > OF(0.0) {
                process_limited(&mut total_amount, &mut time, &inner.data, inner.repeat_count)
            }
        }
        CycleSeq::Inf(inner) => {
            if time > OF(0.0) {
                process_infinite(&mut total_amount, &mut time, &inner.data)
            }
        }
        CycleSeq::LimInf(inner) => {
            if time > OF(0.0) {
                process_limited(&mut total_amount, &mut time, &inner.p1_data, inner.p1_repeat_count)
            }
            if time > OF(0.0) {
                process_infinite(&mut total_amount, &mut time, &inner.p2_data)
            }
        }
        CycleSeq::LimSinInf(inner) => {
            if time > OF(0.0) {
                process_limited(&mut total_amount, &mut time, &inner.p1_data, inner.p1_repeat_count)
            }
            if time > OF(0.0) {
                process_single(&mut total_amount, &mut time, &inner.p2_data)
            }
            if time > OF(0.0) {
                process_infinite(&mut total_amount, &mut time, &inner.p3_data)
            }
        }
        CycleSeq::LoopLimSin(inner) => {
            if time > OF(0.0) {
                // Calculate total "tail time" for whole looped sequence. Data format implies that
                // output can be different, so theoretically tail from first part can be longer than
                // second part with its tail
                let full_tail_time = (inner.p1_data.tail_time - inner.p2_data.time).max(inner.p2_data.tail_time);
                let full_time = inner.p1_data.time * inner.p1_repeat_count as f64 + inner.p2_data.time;
                // Process full loop repeats
                let full_repeats = get_count_full_repeats(time, full_time, full_tail_time);
                total_amount += inner.p1_data.output.get_amount_sum() * AttrVal::from(inner.p1_repeat_count) * full_repeats;
                total_amount += inner.p2_data.output.get_amount_sum() * full_repeats;
                time -= full_time * full_repeats;
                while time >= OF(0.0) {
                    let mut p1_remaining_repeats = inner.p1_repeat_count;
                    // Process as many full part 1 repeats as time can fit
                    let p1_repeats = inner.p1_repeat_count.min(
                        get_count_full_repeats(time, inner.p1_data.time, inner.p1_data.tail_time).into_inner() as Count,
                    );
                    total_amount += inner.p1_data.output.get_amount_sum() * AttrVal::from(p1_repeats);
                    time -= inner.p1_data.time * AttrVal::from(p1_repeats);
                    p1_remaining_repeats -= p1_repeats;
                    // Process partial part 1 repeats
                    while time >= OF(0.0) && p1_remaining_repeats > 0 {
                        total_amount += inner.p1_data.output.get_amount_sum_by_time(time);
                        time -= inner.p1_data.time;
                    }
                    // Process partial part 2
                    if time >= OF(0.0) {
                        total_amount += inner.p2_data.output.get_amount_sum_by_time(time);
                        time -= inner.p2_data.time;
                    }
                    // Outer while loop is for cases of really long tails, which never happen in EVE
                    // but can happen in current data format
                }
            }
        }
    }
    Some(total_amount)
}

fn process_single<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    match *time >= data.time + data.tail_time {
        true => *total_amount += data.output.get_amount_sum(),
        false => *total_amount += data.output.get_amount_sum_by_time(*time),
    }
    *time -= data.time;
}

fn process_limited<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>, repeat_count: Count)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    let full_repeats = repeat_count.min(get_count_full_repeats(*time, data.time, data.tail_time).into_inner() as Count);
    *total_amount += data.output.get_amount_sum() * AttrVal::from(full_repeats);
    let mut remaining_repeats = repeat_count - full_repeats;
    while *time >= OF(0.0) && remaining_repeats > 0 {
        *total_amount += data.output.get_amount_sum_by_time(*time);
        *time -= data.time;
        remaining_repeats -= 1;
    }
}

fn process_infinite<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    let full_repeats = get_count_full_repeats(*time, data.time, data.tail_time);
    *total_amount += data.output.get_amount_sum() * full_repeats;
    *time -= data.time * full_repeats;
    while *time >= OF(0.0) {
        *total_amount += data.output.get_amount_sum_by_time(*time);
        *time -= data.time;
    }
}

fn get_count_full_repeats(time: AttrVal, cycle_time: AttrVal, cycle_tail_time: AttrVal) -> AttrVal {
    let time_no_tail = time - cycle_tail_time;
    if time_no_tail < cycle_time {
        return OF(0.0);
    }
    trunc_unerr(time_no_tail / cycle_time)
}
