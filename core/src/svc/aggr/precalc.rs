use crate::{
    def::{AttrVal, Count, OF},
    svc::{
        cycle::{CycleDataFull, CycleSeq},
        output::Output,
    },
    util::{ConvertExtend, trunc_unerr},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartData<T>
where
    T: Copy,
{
    // Time it takes per cycle in this part
    pub(super) time: AttrVal,
    // After "time" part is complete, it takes this time to finish with output
    pub(super) tail_time: AttrVal,
    pub(super) output: Output<T>,
}

impl<T> ConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            tail_time: (xt.get_completion_time() - self.time).max(OF(0.0)),
            output: xt,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Precalculated data processing
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn aggr_precalc_by_time<T>(precalc: CycleSeq<AggrPartData<T>>, mut time: AttrVal) -> T
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    let mut total_amount = T::default();
    match precalc {
        CycleSeq::Lim(inner) => {
            process_limited_regular(&mut total_amount, &mut time, &inner.data, inner.repeat_count);
        }
        CycleSeq::Inf(inner) => {
            process_infinite_regular(&mut total_amount, &mut time, &inner.data);
        }
        CycleSeq::LimInf(inner) => {
            process_limited_regular(&mut total_amount, &mut time, &inner.p1_data, inner.p1_repeat_count);
            process_infinite_regular(&mut total_amount, &mut time, &inner.p2_data);
        }
        CycleSeq::LimSinInf(inner) => {
            process_limited_regular(&mut total_amount, &mut time, &inner.p1_data, inner.p1_repeat_count);
            process_single_regular(&mut total_amount, &mut time, &inner.p2_data);
            process_infinite_regular(&mut total_amount, &mut time, &inner.p3_data);
        }
        CycleSeq::LoopLimSin(inner) => {
            if time >= OF(0.0) {
                // Calculate total "tail time" for whole looped sequence. Data format implies that
                // output can be different, so theoretically tail from first part can be longer than
                // second part with its tail
                let full_tail_time = (inner.p1_data.tail_time - inner.p2_data.time).max(inner.p2_data.tail_time);
                let full_time = inner.p1_data.time * inner.p1_repeat_count as f64 + inner.p2_data.time;
                // Process full loop repeats
                let full_repeats = get_count_full_repeats(time, full_time, full_tail_time);
                total_amount +=
                    inner.p1_data.output.get_amount_sum() * AttrVal::from(inner.p1_repeat_count) * full_repeats;
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
    total_amount
}

fn process_single_regular<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    if *time < OF(0.0) {
        return;
    }
    match *time >= data.time + data.tail_time {
        true => *total_amount += data.output.get_amount_sum(),
        false => *total_amount += data.output.get_amount_sum_by_time(*time),
    }
    *time -= data.time;
}

fn process_limited_regular<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>, repeat_limit: Count)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    if *time < OF(0.0) {
        return;
    }
    let full_repeats = repeat_limit.min(get_count_full_repeats(*time, data.time, data.tail_time).into_inner() as Count);
    *total_amount += data.output.get_amount_sum() * AttrVal::from(full_repeats);
    let mut remaining_repeats = repeat_limit - full_repeats;
    while *time >= OF(0.0) && remaining_repeats > 0 {
        *total_amount += data.output.get_amount_sum_by_time(*time);
        *time -= data.time;
        remaining_repeats -= 1;
    }
}

fn process_infinite_regular<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    if *time < OF(0.0) {
        return;
    }
    let full_repeats = get_count_full_repeats(*time, data.time, data.tail_time);
    *total_amount += data.output.get_amount_sum() * full_repeats;
    *time -= data.time * full_repeats;
    while *time >= OF(0.0) {
        *total_amount += data.output.get_amount_sum_by_time(*time);
        *time -= data.time;
    }
}

pub(super) fn get_count_full_repeats(time: AttrVal, cycle_time: AttrVal, cycle_tail_time: AttrVal) -> AttrVal {
    let time_no_tail = time - cycle_tail_time;
    if time_no_tail < cycle_time {
        return OF(0.0);
    }
    trunc_unerr(time_no_tail / cycle_time)
}
