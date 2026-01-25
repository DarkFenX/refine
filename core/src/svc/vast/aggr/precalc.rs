use super::traits::GetDuration;
use crate::{
    num::{Count, PValue, Value},
    svc::{
        cycle::{CycleDataFull, CycleSeq},
        output::Output,
    },
    util::LibConvertExtend,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartData<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) duration: PValue,
    // After duration part is complete, it takes this duration to finish with output
    pub(super) tail_duration: PValue,
    pub(super) output: Output<T>,
}

impl<T> LibConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy + GetDuration,
{
    fn lib_convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            duration: self.duration,
            tail_duration: PValue::from_value_clamped(xt.get_completion_duration() - self.duration),
            output: xt,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Precalculated data processing
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn aggr_precalc_by_time<T>(precalc: CycleSeq<AggrPartData<T>>, ptime: PValue) -> T
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    // Locally time can go negative
    let mut time = ptime.into_value();
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
            if time >= Value::ZERO {
                // Calculate total "tail time" for whole looped sequence. Data format implies that
                // output can be different, so theoretically tail from first part can be longer than
                // second part with its tail
                let full_tail_duration = inner
                    .p2_data
                    .tail_duration
                    .max_value(inner.p1_data.tail_duration - inner.p2_data.duration);
                let full_duration =
                    inner.p1_data.duration * inner.p1_repeat_count.into_pvalue() + inner.p2_data.duration;
                // Process full loop repeats
                let full_repeats = get_full_repeats_count(time, full_duration, full_tail_duration).into_pvalue();
                total_amount +=
                    inner.p1_data.output.get_amount_sum() * inner.p1_repeat_count.into_pvalue() * full_repeats;
                total_amount += inner.p2_data.output.get_amount_sum() * full_repeats;
                time -= full_duration * full_repeats;
                while time >= Value::ZERO {
                    let mut p1_remaining_repeats = inner.p1_repeat_count;
                    // Process as many full part 1 repeats as time can fit
                    let p1_repeats = inner.p1_repeat_count.min(get_full_repeats_count(
                        time,
                        inner.p1_data.duration,
                        inner.p1_data.tail_duration,
                    ));
                    total_amount += inner.p1_data.output.get_amount_sum() * p1_repeats.into_pvalue();
                    time -= inner.p1_data.duration * p1_repeats.into_pvalue();
                    p1_remaining_repeats -= p1_repeats;
                    // Process partial part 1 repeats
                    while time >= Value::ZERO && p1_remaining_repeats > Count::ZERO {
                        total_amount += inner
                            .p1_data
                            .output
                            .get_amount_sum_by_time(PValue::from_value_unchecked(time));
                        time -= inner.p1_data.duration;
                    }
                    // Process partial part 2
                    if time >= Value::ZERO {
                        total_amount += inner
                            .p2_data
                            .output
                            .get_amount_sum_by_time(PValue::from_value_unchecked(time));
                        time -= inner.p2_data.duration;
                    }
                    // Outer while loop is for cases of really long tails, which never happen in EVE
                    // but can happen in current data format
                }
            }
        }
    }
    total_amount
}

fn process_single_regular<T>(total_amount: &mut T, time: &mut Value, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    let ptime = match *time < Value::ZERO {
        true => return,
        false => PValue::from_value_unchecked(*time),
    };
    match ptime >= data.duration + data.tail_duration {
        true => *total_amount += data.output.get_amount_sum(),
        false => *total_amount += data.output.get_amount_sum_by_time(ptime),
    }
    *time -= data.duration;
}

fn process_limited_regular<T>(total_amount: &mut T, time: &mut Value, data: &AggrPartData<T>, repeat_limit: Count)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = repeat_limit.min(get_full_repeats_count(*time, data.duration, data.tail_duration));
    *total_amount += data.output.get_amount_sum() * full_repeats.into_pvalue();
    let mut remaining_repeats = repeat_limit - full_repeats;
    while *time >= Value::ZERO && remaining_repeats > Count::ZERO {
        let ptime = PValue::from_value_unchecked(*time);
        *total_amount += data.output.get_amount_sum_by_time(ptime);
        *time -= data.duration;
        remaining_repeats -= Count::ONE;
    }
}

fn process_infinite_regular<T>(total_amount: &mut T, time: &mut Value, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = get_full_repeats_count(*time, data.duration, data.tail_duration).into_pvalue();
    *total_amount += data.output.get_amount_sum() * full_repeats;
    *time -= data.duration * full_repeats;
    while *time >= Value::ZERO {
        let ptime = PValue::from_value_unchecked(*time);
        *total_amount += data.output.get_amount_sum_by_time(ptime);
        *time -= data.duration;
    }
}

pub(super) fn get_full_repeats_count(time: Value, cycle_duration: PValue, cycle_tail_duration: PValue) -> Count {
    let time_no_tail = time - cycle_tail_duration;
    let time_no_tail = match time_no_tail < Value::ZERO {
        true => return Count::ZERO,
        false => PValue::from_value_unchecked(time_no_tail),
    };
    Count::from_pvalue_trunced(time_no_tail / cycle_duration)
}
