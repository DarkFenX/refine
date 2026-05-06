use super::{accum::SeqInstanceAccum, traits::InstanceDuration};
use crate::{
    num::{Count, PValue, Value},
    svc::{
        cycle::{CycleDtHard, CycleSeq},
        output::Output,
    },
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataTail<T>
where
    T: Copy,
{
    // Cycle duration + soft downtime duration
    pub(super) cycle_main_duration: PValue,
    // After main duration part is complete, it takes this duration to finish with output
    pub(super) cycle_tail_duration: Option<PValue>,
    pub(super) output: Output<T>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Precalculated data processing
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn aggr_by_time<T, A>(
    cseq: CycleSeq<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
    accum: &mut A,
    ptime: PValue,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Locally time can go negative
    let mut time = ptime.into_value();
    match cseq {
        CycleSeq::Lim(inner) => {
            process_limited_regular(accum, &mut time, &inner.data, inner.repeat_count, chance_mult);
        }
        CycleSeq::Inf(inner) => match inner.dt_hard {
            Some(dt_hard) => {
                process_infinite_dt_hard(accum, &mut time, &inner.data, dt_hard, chance_mult);
            }
            None => {
                process_infinite_regular(accum, &mut time, &inner.data, chance_mult);
            }
        },
        CycleSeq::LimInf(inner) => {
            process_limited_regular(accum, &mut time, &inner.p1_data, inner.p1_repeat_count, chance_mult);
            process_infinite_regular(accum, &mut time, &inner.p2_data, chance_mult);
        }
        CycleSeq::LimSinInf(inner) => {
            process_limited_regular(accum, &mut time, &inner.p1_data, inner.p1_repeat_count, chance_mult);
            process_single_regular(accum, &mut time, &inner.p2_data, chance_mult);
            process_infinite_regular(accum, &mut time, &inner.p3_data, chance_mult);
        }
        CycleSeq::LoopLimSin(inner) => {
            process_loop_lim_sin(
                accum,
                &mut time,
                &inner.p1_data,
                inner.p1_repeat_count,
                &inner.p2_data,
                chance_mult,
            );
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
    match ptime >= data.cycle_main_duration + data.cycle_tail_duration {
        true => accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count(),
        ),
        false => process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE),
    }
    *time -= data.cycle_main_duration;
}

fn process_limited_regular<T, A>(
    accum: &mut A,
    time: &mut Value,
    data: &AggrPartDataTail<T>,
    repeat_limit: Count,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeats = repeat_limit.min(get_full_repeats_count(
        *time,
        data.cycle_main_duration,
        data.cycle_tail_duration,
    ));
    if full_repeats > Count::ZERO {
        accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count() * full_repeats,
        );
        *time -= data.cycle_main_duration * full_repeats.into_pvalue();
    }
    let mut remaining_repeats = repeat_limit - full_repeats;
    while *time >= Value::ZERO && remaining_repeats > Count::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE);
        *time -= data.cycle_main_duration;
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
    let full_repeats = get_full_repeats_count(*time, data.cycle_main_duration, data.cycle_tail_duration);
    if full_repeats > Count::ZERO {
        accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count() * full_repeats,
        );
        *time -= data.cycle_main_duration * full_repeats.into_pvalue();
    }
    while *time >= Value::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE);
        *time -= data.cycle_main_duration;
    }
}

fn process_infinite_dt_hard<T, A>(
    accum: &mut A,
    time: &mut Value,
    data: &AggrPartDataTail<T>,
    dt_hard: CycleDtHard,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Calculate how many full durations we can fit into given time, considering hard
    // downtimes
    let cycle_full_duration = data.cycle_main_duration + dt_hard.duration;
    let mut full_repeats = Count::from_value_trunced(*time / cycle_full_duration);
    *time -= cycle_full_duration * full_repeats.into_pvalue();
    if *time >= data.cycle_main_duration.into_value() {
        full_repeats += Count::ONE;
        *time -= cycle_full_duration;
    }
    if full_repeats > Count::ZERO {
        // Hard downtimes cut output tails. If output has a tail (it couldn't be fit
        // into main duration), process cycle like partial
        match data.cycle_tail_duration {
            Some(_) => process_incomplete_cycle(
                accum,
                data.cycle_main_duration.into_value(),
                &data.output,
                chance_mult,
                full_repeats,
            ),
            None => accum.add_instance(
                data.output.get_instance(),
                chance_mult,
                data.output.get_instance_count() * full_repeats,
            ),
        }
    }
    // If there is still time left, process cycles which only partially fit
    while *time >= Value::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE);
        *time -= data.cycle_main_duration;
    }
}

fn process_loop_lim_sin<T, A>(
    accum: &mut A,
    time: &mut Value,
    p1_data: &AggrPartDataTail<T>,
    p1_repeat_count: Count,
    p2_data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Calculate total "tail time" for whole looped sequence. Data format implies that
    // output can be different, so theoretically tail from first part can be longer than
    // second part with its tail
    let full_tail_duration = p2_data
        .cycle_tail_duration
        .max_value(p1_data.cycle_tail_duration - p2_data.cycle_main_duration);
    let full_duration = p1_data.cycle_main_duration * p1_repeat_count.into_pvalue() + p2_data.cycle_main_duration;
    // Process full loop repeats
    let full_repeats = get_full_repeats_count(*time, full_duration, full_tail_duration);
    if full_repeats > Count::ZERO {
        accum.add_instance(
            p1_data.output.get_instance(),
            chance_mult,
            p1_data.output.get_instance_count() * p1_repeat_count * full_repeats,
        );
        accum.add_instance(
            p2_data.output.get_instance(),
            chance_mult,
            p2_data.output.get_instance_count() * full_repeats,
        );
        *time -= full_duration * full_repeats.into_pvalue();
    }
    while *time >= Value::ZERO {
        let mut p1_remaining_repeats = p1_repeat_count;
        // Process as many full part 1 repeats as time can fit
        let p1_repeats = p1_repeat_count.min(get_full_repeats_count(
            *time,
            p1_data.cycle_main_duration,
            p1_data.cycle_tail_duration,
        ));
        if p1_repeats > Count::ZERO {
            accum.add_instance(
                p1_data.output.get_instance(),
                chance_mult,
                p1_data.output.get_instance_count() * p1_repeats,
            );
            *time -= p1_data.cycle_main_duration * p1_repeats.into_pvalue();
            p1_remaining_repeats -= p1_repeats;
        }
        // Process partial part 1 repeats
        while *time >= Value::ZERO && p1_remaining_repeats > Count::ZERO {
            process_incomplete_cycle(accum, *time, &p1_data.output, chance_mult, Count::ONE);
            *time -= p1_data.cycle_main_duration;
            p1_remaining_repeats -= Count::ONE;
        }
        // Process partial part 2
        if *time >= Value::ZERO {
            process_incomplete_cycle(accum, *time, &p2_data.output, chance_mult, Count::ONE);
            *time -= p2_data.cycle_main_duration;
        }
        // Outer while loop is for cases of really long tails, which never happen in EVE
        // but can happen in current data format
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
// Applicable only to sequences without hard downtime
pub(super) fn get_full_repeats_count(
    time: Value,
    cycle_main_duration: PValue,
    cycle_tail_duration: Option<PValue>,
) -> Count {
    let time_no_tail = match cycle_tail_duration {
        Some(cycle_tail_duration) => time - cycle_tail_duration,
        None => time,
    };
    let time_no_tail = match time_no_tail < Value::ZERO {
        true => return Count::ZERO,
        false => PValue::from_value_unchecked(time_no_tail),
    };
    Count::from_pvalue_trunced(time_no_tail / cycle_main_duration)
}

// Cheap processing works only when cycle + its tail (output / instance durations) fit into time;
// this function has more expensive, but more accurate processing for case when there is not enough
// time to fit everything
pub(super) fn process_incomplete_cycle<T, A>(
    accum: &mut A,
    mut time: Value,
    output: &Output<T>,
    chance_mult: Option<PValue>,
    repeats: Count,
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
        accum.add_instance(instance_data.instance, chance_mult, repeats)
    }
}
