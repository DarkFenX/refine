use super::{
    accum::SeqInstanceAccum,
    shared::{
        AggrPartDataTail, get_full_repeat_count, process_full_cycle_with_cutoff, process_full_loop_lim_sin_with_cutoff,
        process_incomplete_cycle,
    },
    traits::InstanceDuration,
};
use crate::{
    num::{Count, PValue, Value},
    svc::cycle::{CycleDtHard, CycleSeq},
};

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
    match cseq {
        CycleSeq::Lim(inner) => process_limited_regular(
            accum,
            &mut ptime.into_value(),
            &inner.data,
            inner.repeat_count,
            chance_mult,
        ),
        CycleSeq::Inf(inner) => match inner.dt_hard {
            Some(dt_hard) => process_infinite_hard_dt(accum, ptime, &inner.data, dt_hard, chance_mult),
            None => process_infinite_regular(accum, &mut ptime.into_value(), &inner.data, chance_mult),
        },
        CycleSeq::LimInf(inner) => {
            let mut time = ptime.into_value();
            process_limited_regular(accum, &mut time, &inner.p1_data, inner.p1_repeat_count, chance_mult);
            process_infinite_regular(accum, &mut time, &inner.p2_data, chance_mult);
        }
        CycleSeq::LimSinInf(inner) => {
            let mut time = ptime.into_value();
            process_limited_regular(accum, &mut time, &inner.p1_data, inner.p1_repeat_count, chance_mult);
            process_single_regular(accum, &mut time, &inner.p2_data, chance_mult);
            process_infinite_regular(accum, &mut time, &inner.p3_data, chance_mult);
        }
        CycleSeq::LoopLimSin(inner) => match inner.dt_hard {
            Some(dt_hard) => process_loop_lim_sin_hard_dt(
                accum,
                ptime,
                &inner.p1_data,
                inner.p1_repeat_count,
                &inner.p2_data,
                dt_hard,
                chance_mult,
            ),
            None => process_loop_lim_sin_regular(
                accum,
                ptime,
                &inner.p1_data,
                inner.p1_repeat_count,
                &inner.p2_data,
                chance_mult,
            ),
        },
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
    match ptime >= data.get_duration_with_tail() {
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
    let full_repeat_count = repeat_limit.min(get_full_repeat_count(
        *time,
        data.cycle_main_duration,
        data.cycle_tail_duration,
    ));
    if full_repeat_count > Count::ZERO {
        accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count() * full_repeat_count,
        );
        *time -= data.cycle_main_duration * full_repeat_count.into_pvalue();
    }
    let mut remaining_repeat_count = repeat_limit - full_repeat_count;
    while *time >= Value::ZERO && remaining_repeat_count > Count::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE);
        *time -= data.cycle_main_duration;
        remaining_repeat_count -= Count::ONE;
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
    let full_repeat_count = get_full_repeat_count(*time, data.cycle_main_duration, data.cycle_tail_duration);
    if full_repeat_count > Count::ZERO {
        accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count() * full_repeat_count,
        );
        *time -= data.cycle_main_duration * full_repeat_count.into_pvalue();
    }
    while *time >= Value::ZERO {
        process_incomplete_cycle(accum, *time, &data.output, chance_mult, Count::ONE);
        *time -= data.cycle_main_duration;
    }
}

fn process_infinite_hard_dt<T, A>(
    accum: &mut A,
    ptime: PValue,
    data: &AggrPartDataTail<T>,
    dt_hard: CycleDtHard,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut time = ptime.into_value();
    // Calculate how many full durations we can fit into given time, considering hard downtimes
    let full_duration = data.cycle_main_duration + dt_hard.duration;
    let mut full_repeat_count = Count::from_value_trunced(time / full_duration);
    time -= full_duration * full_repeat_count.into_pvalue();
    if time >= data.cycle_main_duration.into_value() {
        full_repeat_count += Count::ONE;
        time -= full_duration;
    }
    if full_repeat_count > Count::ZERO {
        process_full_cycle_with_cutoff(accum, data, chance_mult, full_repeat_count);
    }
    // If there is still time left, process cycles which only partially fit
    while time >= Value::ZERO {
        process_incomplete_cycle(accum, time, &data.output, chance_mult, Count::ONE);
        time -= data.cycle_main_duration;
    }
}

fn process_loop_lim_sin_regular<T, A>(
    accum: &mut A,
    ptime: PValue,
    p1_data: &AggrPartDataTail<T>,
    p1_repeat_count: Count,
    p2_data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut time = ptime.into_value();
    // Calculate total "tail time" for whole looped sequence. Data format implies that output can be
    // different, so theoretically tail from first part can be longer than second part with its tail
    let full_tail_duration = get_loop_lim_sin_full_tail_duration(p1_data, p2_data);
    let full_duration = p1_data.cycle_main_duration * p1_repeat_count.into_pvalue() + p2_data.cycle_main_duration;
    // Process full loop repeats
    let full_repeat_count = get_full_repeat_count(time, full_duration, full_tail_duration);
    if full_repeat_count > Count::ZERO {
        accum.add_instance(
            p1_data.output.get_instance(),
            chance_mult,
            p1_data.output.get_instance_count() * p1_repeat_count * full_repeat_count,
        );
        accum.add_instance(
            p2_data.output.get_instance(),
            chance_mult,
            p2_data.output.get_instance_count() * full_repeat_count,
        );
        time -= full_duration * full_repeat_count.into_pvalue();
    }
    // While loop instead of if is for cases of really long tails, which never happen in EVE but can
    // happen in current data format
    while time >= Value::ZERO {
        process_loop_lim_sin_incomplete(accum, &mut time, p1_data, p1_repeat_count, p2_data, chance_mult);
    }
}

fn process_loop_lim_sin_hard_dt<T, A>(
    accum: &mut A,
    ptime: PValue,
    p1_data: &AggrPartDataTail<T>,
    p1_repeat_count: Count,
    p2_data: &AggrPartDataTail<T>,
    hard_dt: CycleDtHard,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut time = ptime.into_value();
    // Calculate how many full inner durations (time in-between hard downtimes, when output
    // instances can apply) can fit into given time, and modify time left
    let loop_inner_duration = p1_data.cycle_main_duration * p1_repeat_count.into_pvalue() + p2_data.cycle_main_duration;
    let loop_full_duration = loop_inner_duration + hard_dt.duration;
    let mut loop_full_repeat_count = Count::from_value_trunced(time / loop_full_duration);
    time -= loop_full_duration * loop_full_repeat_count.into_pvalue();
    if time >= loop_inner_duration.into_value() {
        loop_full_repeat_count += Count::ONE;
        time -= loop_full_duration;
    }
    // Apply full loops
    if loop_full_repeat_count > Count::ZERO {
        process_full_loop_lim_sin_with_cutoff(
            accum,
            p1_data,
            p1_repeat_count,
            p2_data,
            chance_mult,
            loop_inner_duration,
            loop_full_repeat_count,
        );
    }
    // Apply partial loop
    if time >= Value::ZERO {
        process_loop_lim_sin_incomplete(accum, &mut time, p1_data, p1_repeat_count, p2_data, chance_mult);
    }
}

fn process_loop_lim_sin_incomplete<T, A>(
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
    let mut p1_remaining_repeat_count = p1_repeat_count;
    // Process as many full part 1 repeats as time can fit
    let p1_full_repeat_count = p1_repeat_count.min(get_full_repeat_count(
        *time,
        p1_data.cycle_main_duration,
        p1_data.cycle_tail_duration,
    ));
    if p1_full_repeat_count > Count::ZERO {
        accum.add_instance(
            p1_data.output.get_instance(),
            chance_mult,
            p1_data.output.get_instance_count() * p1_full_repeat_count,
        );
        *time -= p1_data.cycle_main_duration * p1_full_repeat_count.into_pvalue();
        p1_remaining_repeat_count -= p1_full_repeat_count;
    }
    // Process partial part 1 repeats
    while *time >= Value::ZERO && p1_remaining_repeat_count > Count::ZERO {
        process_incomplete_cycle(accum, *time, &p1_data.output, chance_mult, Count::ONE);
        *time -= p1_data.cycle_main_duration;
        p1_remaining_repeat_count -= Count::ONE;
    }
    // Process partial part 2
    if *time >= Value::ZERO {
        process_incomplete_cycle(accum, *time, &p2_data.output, chance_mult, Count::ONE);
        *time -= p2_data.cycle_main_duration;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_loop_lim_sin_full_tail_duration<T>(
    p1_data: &AggrPartDataTail<T>,
    p2_data: &AggrPartDataTail<T>,
) -> Option<PValue>
where
    T: Copy,
{
    match (p1_data.cycle_tail_duration, p2_data.cycle_tail_duration) {
        (Some(p1_tail_duration), Some(p2_tail_duration)) => {
            let p2_duration_with_tail = p2_data.cycle_main_duration + p2_tail_duration;
            match p1_tail_duration > p2_duration_with_tail {
                true => Some(PValue::from_value_unchecked(p1_tail_duration - p2_duration_with_tail)),
                false => Some(p2_tail_duration),
            }
        }
        (Some(p1_tail_duration), None) => match p1_tail_duration > p2_data.cycle_main_duration {
            true => Some(PValue::from_value_unchecked(
                p1_tail_duration - p2_data.cycle_main_duration,
            )),
            false => None,
        },
        (None, Some(p2_tail_duration)) => Some(p2_tail_duration),
        (None, None) => None,
    }
}
