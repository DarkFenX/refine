use super::{accum::SeqInstanceAccum, traits::InstanceDuration};
use crate::{
    num::{Count, PValue, Value},
    svc::{cycle::CycleSeq, output::Output},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataTail<T>
where
    T: Copy,
{
    // Duration it takes per cycle in this part
    pub(super) cycle_duration: PValue,
    // After duration part is complete, it takes this duration to finish with output
    pub(super) cycle_tail_duration: PValue,
    pub(super) output: Output<T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Precalculated data processing
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn aggr_by_time<T, A>(
    precalc: CycleSeq<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
    accum: &mut A,
    ptime: PValue,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Locally time can go negative
    let mut time = ptime.into_value();
    match precalc {
        CycleSeq::Lim(inner) => {
            process_limited_regular(accum, &mut time, &inner.data, chance_mult, inner.repeat_count);
        }
        CycleSeq::Inf(inner) => {
            process_infinite_regular(accum, &mut time, &inner.data, chance_mult);
        }
        CycleSeq::LimInf(inner) => {
            process_limited_regular(accum, &mut time, &inner.p1_data, chance_mult, inner.p1_repeat_count);
            process_infinite_regular(accum, &mut time, &inner.p2_data, chance_mult);
        }
        CycleSeq::LimSinInf(inner) => {
            process_limited_regular(accum, &mut time, &inner.p1_data, chance_mult, inner.p1_repeat_count);
            process_single_regular(accum, &mut time, &inner.p2_data, chance_mult);
            process_infinite_regular(accum, &mut time, &inner.p3_data, chance_mult);
        }
        CycleSeq::LoopLimSin(inner) => {
            if time >= Value::ZERO {
                // Calculate total "tail time" for whole looped sequence. Data format implies that
                // output can be different, so theoretically tail from first part can be longer than
                // second part with its tail
                let full_tail_duration = inner
                    .p2_data
                    .cycle_tail_duration
                    .max_value(inner.p1_data.cycle_tail_duration - inner.p2_data.cycle_duration);
                let full_duration =
                    inner.p1_data.cycle_duration * inner.p1_repeat_count.into_pvalue() + inner.p2_data.cycle_duration;
                // Process full loop repeats
                let full_repeats = get_full_repeats_count(time, full_duration, full_tail_duration);
                accum.add_instance(
                    inner.p1_data.output.get_instance(),
                    chance_mult,
                    inner.p1_data.output.get_instance_count() * inner.p1_repeat_count * full_repeats,
                );
                accum.add_instance(
                    inner.p2_data.output.get_instance(),
                    chance_mult,
                    inner.p2_data.output.get_instance_count() * full_repeats,
                );
                time -= full_duration * full_repeats.into_pvalue();
                while time >= Value::ZERO {
                    let mut p1_remaining_repeats = inner.p1_repeat_count;
                    // Process as many full part 1 repeats as time can fit
                    let p1_repeats = inner.p1_repeat_count.min(get_full_repeats_count(
                        time,
                        inner.p1_data.cycle_duration,
                        inner.p1_data.cycle_tail_duration,
                    ));
                    accum.add_instance(
                        inner.p1_data.output.get_instance(),
                        chance_mult,
                        inner.p1_data.output.get_instance_count() * p1_repeats,
                    );
                    time -= inner.p1_data.cycle_duration * p1_repeats.into_pvalue();
                    p1_remaining_repeats -= p1_repeats;
                    // Process partial part 1 repeats
                    while time >= Value::ZERO && p1_remaining_repeats > Count::ZERO {
                        process_incomplete_cycle(accum, time, &inner.p1_data.output, chance_mult);
                        time -= inner.p1_data.cycle_duration;
                    }
                    // Process partial part 2
                    if time >= Value::ZERO {
                        process_incomplete_cycle(accum, time, &inner.p2_data.output, chance_mult);
                        time -= inner.p2_data.cycle_duration;
                    }
                    // Outer while loop is for cases of really long tails, which never happen in EVE
                    // but can happen in current data format
                }
            }
        }
    }
}

fn process_single_regular<T, A>(
    accum: &mut A,
    time: &mut Value,
    data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let ptime = match *time < Value::ZERO {
        true => return,
        false => PValue::from_value_unchecked(*time),
    };
    match ptime >= data.cycle_duration + data.cycle_tail_duration {
        true => accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count(),
        ),
        false => process_incomplete_cycle(accum, *time, &data.output, chance_mult),
    }
    *time -= data.cycle_duration;
}

fn process_limited_regular<T, A>(
    accum: &mut A,
    time: &mut Value,
    data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
    repeat_limit: Count,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = repeat_limit.min(get_full_repeats_count(
        *time,
        data.cycle_duration,
        data.cycle_tail_duration,
    ));
    accum.add_instance(
        data.output.get_instance(),
        chance_mult,
        data.output.get_instance_count() * full_repeats,
    );
    *time -= data.cycle_duration * full_repeats.into_pvalue();
    let mut remaining_repeats = repeat_limit - full_repeats;
    while *time >= Value::ZERO && remaining_repeats > Count::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult);
        *time -= data.cycle_duration;
        remaining_repeats -= Count::ONE;
    }
}

fn process_infinite_regular<T, A>(
    accum: &mut A,
    time: &mut Value,
    data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = get_full_repeats_count(*time, data.cycle_duration, data.cycle_tail_duration);
    accum.add_instance(
        data.output.get_instance(),
        chance_mult,
        data.output.get_instance_count() * full_repeats,
    );
    *time -= data.cycle_duration * full_repeats.into_pvalue();
    while *time >= Value::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult);
        *time -= data.cycle_duration;
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

// Cheap processing works only when cycle + its tail (output / instance durations) fit into time;
// this function has more expensive, but more accurate processing for case when there is not enough
// time to fit everything
pub(super) fn process_incomplete_cycle<T, A>(
    accum: &mut A,
    mut time: Value,
    output: &Output<T>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Time here is supposed to be modified only locally, and should not affect time out-of-cycle
    // time tracking
    for mut instance_data in output.into_instance_iter() {
        time -= instance_data.time_passed;
        let ptime = match time >= Value::ZERO {
            true => PValue::from_value_unchecked(time),
            false => break,
        };
        instance_data.instance.limit_duration(ptime);
        accum.add_instance(instance_data.instance, chance_mult, Count::ONE)
    }
}
