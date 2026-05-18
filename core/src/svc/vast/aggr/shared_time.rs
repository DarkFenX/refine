use super::{
    accum::SeqInstanceAccum,
    shared::{
        AggrHardDtSimple, AggrPartDataTail, get_cycle_tail_duration, get_tailed_cycle_full_repeat_count,
        process_output_of_cycle_with_cutoff, process_output_of_lls_with_cutoff,
    },
    traits::InstanceDuration,
};
use crate::{
    num::{Count, PValue, Value},
    svc::cycle::{CSeqLoopLimSin, CycleSeq},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Precalculated data processing
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn aggr_by_time<I, IA>(
    cseq: CycleSeq<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
    accum: &mut IA,
    ptime: PValue,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeq::Lim(inner) => process_limited_regular(
            accum,
            &mut ptime.into_value(),
            &inner.data,
            inner.repeat_count,
            chance_mult,
        ),
        CycleSeq::Inf(inner) => match inner.hard_dt {
            Some(hard_dt) => process_infinite_hard_dt(accum, ptime, &inner.data, hard_dt, chance_mult),
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
        CycleSeq::LoopLimSin(inner) => match inner.hard_dt {
            Some(_) => process_lls_hard_dt(accum, ptime, inner, chance_mult),
            None => process_lls_regular(accum, ptime, inner, chance_mult),
        },
    }
}

fn process_single_regular<I, IA>(
    accum: &mut IA,
    time: &mut Value,
    data: &AggrPartDataTail<I>,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
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

fn process_limited_regular<I, IA>(
    accum: &mut IA,
    time: &mut Value,
    data: &AggrPartDataTail<I>,
    repeat_limit: Count,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeat_count = repeat_limit.min(get_tailed_cycle_full_repeat_count(
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

fn process_infinite_regular<I, IA>(
    accum: &mut IA,
    time: &mut Value,
    data: &AggrPartDataTail<I>,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    if *time < Value::ZERO {
        return;
    }
    let full_repeat_count =
        get_tailed_cycle_full_repeat_count(*time, data.cycle_main_duration, data.cycle_tail_duration);
    if full_repeat_count > Count::ZERO {
        accum.add_output_full(&data.output, chance_mult, full_repeat_count);
        *time -= data.cycle_main_duration * full_repeat_count.into_pvalue();
    }
    while *time >= Value::ZERO {
        accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, *time);
        *time -= data.cycle_main_duration;
    }
}

fn process_infinite_hard_dt<I, IA>(
    accum: &mut IA,
    ptime: PValue,
    data: &AggrPartDataTail<I>,
    hard_dt: AggrHardDtSimple,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    let mut time = ptime.into_value();
    // Calculate how many full durations we can fit into given time, considering hard downtimes, and
    // calculate remaining time
    let full_duration = data.cycle_main_duration + hard_dt.duration;
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
    // If there is still time left, process cycle which only partially fits. Multiple cycles cannot
    // fit in this case, as hard downtime cuts cycle tails
    if time >= Value::ZERO {
        accum.add_output_time_limited(&data.output, chance_mult, Count::ONE, time);
    }
}

fn process_lls_regular<I, IA>(
    accum: &mut IA,
    ptime: PValue,
    cseq: CSeqLoopLimSin<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    let mut time = ptime.into_value();
    // Data format implies that completion duration of outputs in different parts can be different,
    // but here we assume that it's the same. This assumption upholds, since part outputs are just
    // strength-multiplied copies of base output. If this assumption is broken, need to consider
    // tail of the first part (which could be longer than main duration + tail of the second part).
    let loop_tail_duration = get_cycle_tail_duration(
        cseq.p2_data.cycle_main_duration,
        cseq.p2_data.output.get_completion_duration(),
    );
    let loop_inner_duration = cseq.get_full_duration();
    // Process full loop repeats
    let full_repeat_count = get_tailed_cycle_full_repeat_count(time, loop_inner_duration, loop_tail_duration);
    if full_repeat_count > Count::ZERO {
        accum.add_output_full(
            &cseq.p1_data.output,
            chance_mult,
            full_repeat_count * cseq.p1_repeat_count,
        );
        accum.add_output_full(&cseq.p2_data.output, chance_mult, full_repeat_count);
        time -= loop_inner_duration * full_repeat_count.into_pvalue();
    }
    // While loop instead of if is for cases of really long tails, which never happen in EVE but can
    // happen in current data format
    while time >= Value::ZERO {
        process_lls_incomplete(accum, &mut time, &cseq, chance_mult);
    }
}

fn process_lls_hard_dt<I, IA>(
    accum: &mut IA,
    ptime: PValue,
    cseq: CSeqLoopLimSin<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    let mut time = ptime.into_value();
    let loop_inner_duration = cseq.get_full_duration();
    let loop_full_duration = loop_inner_duration + cseq.hard_dt.unwrap().duration;
    let loop_full_repeat_count = get_cutoff_cycle_full_repeat_count(time, loop_inner_duration, loop_full_duration);
    // Apply full loops
    if loop_full_repeat_count > Count::ZERO {
        process_output_of_lls_with_cutoff(accum, &cseq, chance_mult, loop_full_repeat_count);
        time -= loop_full_duration * loop_full_repeat_count.into_pvalue();
    }
    // Apply partial loop
    if time >= Value::ZERO {
        process_lls_incomplete(accum, &mut time, &cseq, chance_mult);
    }
}

fn process_lls_incomplete<I, IA>(
    accum: &mut IA,
    time: &mut Value,
    cseq: &CSeqLoopLimSin<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    let mut p1_remaining_repeat_count = cseq.p1_repeat_count;
    // Process as many full part 1 repeats as time can fit
    let p1_full_repeat_count = cseq.p1_repeat_count.min(get_tailed_cycle_full_repeat_count(
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

pub(super) fn get_cutoff_cycle_full_repeat_count(
    mut time: Value,
    cycle_inner_duration: PValue,
    cycle_full_duration: PValue,
) -> Count {
    let mut full_repeat_count = Count::from_value_trunced(time / cycle_full_duration);
    time -= cycle_full_duration * full_repeat_count.into_pvalue();
    if time >= cycle_inner_duration.into_value() {
        full_repeat_count += Count::ONE;
    }
    full_repeat_count
}
