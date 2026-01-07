use crate::{
    misc::{Count, PValue, Value},
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
    // Time it takes per cycle in this part
    pub(super) time: PValue,
    // After "time" part is complete, it takes this time to finish with output
    pub(super) tail_time: PValue,
    pub(super) output: Output<T>,
}

impl<T> LibConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn lib_convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            tail_time: PValue::from_value_clamped(xt.get_completion_time() - self.time),
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
                let full_tail_time = inner
                    .p2_data
                    .tail_time
                    .max_value(inner.p1_data.tail_time - inner.p2_data.time);
                let full_time = inner.p1_data.time * inner.p1_repeat_count.into_pvalue() + inner.p2_data.time;
                // Process full loop repeats
                let full_repeats = get_full_repeats_count(time, full_time, full_tail_time).into_pvalue();
                total_amount +=
                    inner.p1_data.output.get_amount_sum() * inner.p1_repeat_count.into_pvalue() * full_repeats;
                total_amount += inner.p2_data.output.get_amount_sum() * full_repeats;
                time -= full_time * full_repeats;
                while time >= Value::ZERO {
                    let mut p1_remaining_repeats = inner.p1_repeat_count;
                    // Process as many full part 1 repeats as time can fit
                    let p1_repeats = inner.p1_repeat_count.min(get_full_repeats_count(
                        time,
                        inner.p1_data.time,
                        inner.p1_data.tail_time,
                    ));
                    total_amount += inner.p1_data.output.get_amount_sum() * p1_repeats.into_pvalue();
                    time -= inner.p1_data.time * p1_repeats.into_pvalue();
                    p1_remaining_repeats -= p1_repeats;
                    // Process partial part 1 repeats
                    while time >= Value::ZERO && p1_remaining_repeats > Count::ZERO {
                        total_amount += inner
                            .p1_data
                            .output
                            .get_amount_sum_by_time(PValue::from_val_unchecked(time));
                        time -= inner.p1_data.time;
                    }
                    // Process partial part 2
                    if time >= Value::ZERO {
                        total_amount += inner
                            .p2_data
                            .output
                            .get_amount_sum_by_time(PValue::from_val_unchecked(time));
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

fn process_single_regular<T>(total_amount: &mut T, time: &mut Value, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    let ptime = match *time < Value::ZERO {
        true => return,
        false => PValue::from_val_unchecked(*time),
    };
    match ptime >= data.time + data.tail_time {
        true => *total_amount += data.output.get_amount_sum(),
        false => *total_amount += data.output.get_amount_sum_by_time(ptime),
    }
    *time -= data.time;
}

fn process_limited_regular<T>(total_amount: &mut T, time: &mut Value, data: &AggrPartData<T>, repeat_limit: Count)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<PValue, Output = T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = repeat_limit.min(get_full_repeats_count(*time, data.time, data.tail_time));
    *total_amount += data.output.get_amount_sum() * full_repeats.into_pvalue();
    let mut remaining_repeats = repeat_limit - full_repeats;
    while *time >= Value::ZERO && remaining_repeats > Count::ZERO {
        let ptime = PValue::from_val_unchecked(*time);
        *total_amount += data.output.get_amount_sum_by_time(ptime);
        *time -= data.time;
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
    let full_repeats = get_full_repeats_count(*time, data.time, data.tail_time).into_pvalue();
    *total_amount += data.output.get_amount_sum() * full_repeats;
    *time -= data.time * full_repeats;
    while *time >= Value::ZERO {
        let ptime = PValue::from_val_unchecked(*time);
        *total_amount += data.output.get_amount_sum_by_time(ptime);
        *time -= data.time;
    }
}

pub(super) fn get_full_repeats_count(time: Value, cycle_time: PValue, cycle_tail_time: PValue) -> Count {
    let time_no_tail = time - cycle_tail_time;
    let time_no_tail = match time_no_tail < Value::ZERO {
        true => return Count::ZERO,
        false => PValue::from_val_unchecked(time_no_tail),
    };
    Count::from_pvalue_trunced(time_no_tail / cycle_time)
}
