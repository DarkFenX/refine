use super::{
    accum::SeqInstanceAccum,
    shared::{
        AggrPartDataTail, get_full_repeat_count, process_output_of_cycle_with_cutoff,
        process_output_of_lls_cseq_with_cutoff,
    },
    traits::InstanceDuration,
};
use crate::{
    num::{Count, PValue, Value},
    svc::cycle::{CSeqLoopLimSin, CycleDtHard, CycleSeq},
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
            Some(_) => process_loop_lim_sin_hard_dt(accum, ptime, inner, chance_mult),
            None => process_loop_lim_sin_regular(accum, ptime, inner, chance_mult),
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
        true => accum.add_output_full(&data.output, chance_mult, Count::ONE),
        false => accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, *time),
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
        accum.add_output_full(&data.output, chance_mult, full_repeat_count);
        *time -= data.cycle_main_duration * full_repeat_count.into_pvalue();
    }
    let mut remaining_repeat_count = repeat_limit - full_repeat_count;
    while *time >= Value::ZERO && remaining_repeat_count > Count::ZERO {
        accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, *time);
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
        accum.add_output_full(&data.output, chance_mult, full_repeat_count);
        *time -= data.cycle_main_duration * full_repeat_count.into_pvalue();
    }
    while *time >= Value::ZERO {
        accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, *time);
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
    // Calculate how many full durations we can fit into given time, considering hard downtimes, and
    // calculate remaining time
    let full_duration = data.cycle_main_duration + dt_hard.duration;
    let mut full_repeat_count = Count::from_value_trunced(time / full_duration);
    time -= full_duration * full_repeat_count.into_pvalue();
    if time >= data.cycle_main_duration.into_value() {
        full_repeat_count += Count::ONE;
        time -= full_duration;
    }
    // Add full repeats
    if full_repeat_count > Count::ZERO {
        process_output_of_cycle_with_cutoff(accum, data, chance_mult, full_repeat_count);
    }
    // If there is still time left, process cycles which only partially fit
    while time >= Value::ZERO {
        accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, time);
    }
}

fn process_loop_lim_sin_regular<T, A>(
    accum: &mut A,
    ptime: PValue,
    cseq: CSeqLoopLimSin<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut time = ptime.into_value();
    // Calculate total "tail time" for whole looped sequence. Data format implies that output can be
    // different, so theoretically tail from first part can be longer than second part with its tail
    let full_tail_duration = get_loop_lim_sin_full_tail_duration(&cseq.p1_data, &cseq.p2_data);
    let full_inner_duration = cseq.get_inner_duration();
    // Process full loop repeats
    let full_repeat_count = get_full_repeat_count(time, full_inner_duration, full_tail_duration);
    if full_repeat_count > Count::ZERO {
        accum.add_output_full(
            &cseq.p1_data.output,
            chance_mult,
            full_repeat_count * cseq.p1_repeat_count,
        );
        accum.add_output_full(&cseq.p2_data.output, chance_mult, full_repeat_count);
        time -= full_inner_duration * full_repeat_count.into_pvalue();
    }
    // While loop instead of if is for cases of really long tails, which never happen in EVE but can
    // happen in current data format
    while time >= Value::ZERO {
        process_loop_lim_sin_incomplete(accum, &mut time, &cseq, chance_mult);
    }
}

fn process_loop_lim_sin_hard_dt<T, A>(
    accum: &mut A,
    ptime: PValue,
    cseq: CSeqLoopLimSin<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut time = ptime.into_value();
    let loop_inner_duration = cseq.get_inner_duration();
    let loop_full_duration = loop_inner_duration + cseq.dt_hard.unwrap().duration;
    let mut loop_full_repeat_count = Count::from_value_trunced(time / loop_full_duration);
    time -= loop_full_duration * loop_full_repeat_count.into_pvalue();
    if time >= loop_inner_duration.into_value() {
        loop_full_repeat_count += Count::ONE;
        time -= loop_full_duration;
    }
    // Apply full loops
    if loop_full_repeat_count > Count::ZERO {
        process_output_of_lls_cseq_with_cutoff(accum, &cseq, chance_mult, loop_full_repeat_count);
    }
    // Apply partial loop
    if time >= Value::ZERO {
        process_loop_lim_sin_incomplete(accum, &mut time, &cseq, chance_mult);
    }
}

fn process_loop_lim_sin_incomplete<T, A>(
    accum: &mut A,
    time: &mut Value,
    cseq: &CSeqLoopLimSin<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    let mut p1_remaining_repeat_count = cseq.p1_repeat_count;
    // Process as many full part 1 repeats as time can fit
    let p1_full_repeat_count = cseq.p1_repeat_count.min(get_full_repeat_count(
        *time,
        cseq.p1_data.cycle_main_duration,
        cseq.p1_data.cycle_tail_duration,
    ));
    if p1_full_repeat_count > Count::ZERO {
        accum.add_output_full(&cseq.p1_data.output, chance_mult, p1_full_repeat_count);
        *time -= cseq.p1_data.cycle_main_duration * p1_full_repeat_count.into_pvalue();
        p1_remaining_repeat_count -= p1_full_repeat_count;
    }
    // Process partial part 1 repeats
    while *time >= Value::ZERO && p1_remaining_repeat_count > Count::ZERO {
        accum.add_output_time_limited(&cseq.p1_data.output, chance_mult, Count::ONE, *time);
        *time -= cseq.p1_data.cycle_main_duration;
        p1_remaining_repeat_count -= Count::ONE;
    }
    // Process partial part 2
    if *time >= Value::ZERO {
        accum.add_output_time_limited(&cseq.p2_data.output, chance_mult, Count::ONE, *time);
        *time -= cseq.p2_data.cycle_main_duration;
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
