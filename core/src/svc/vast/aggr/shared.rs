use super::{accum::SeqInstanceAccum, traits::InstanceDuration};
use crate::{
    num::{Count, PValue, Value},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(super) fn get_item_ship_limit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    attr_rid: Option<RAttrId>,
) -> Option<PValue> {
    let attr_rid = attr_rid?;
    let fit_uid = ctx.u_data.items.get(item_uid).get_fit_uid()?;
    let ship_uid = ctx.u_data.fits.get(fit_uid).ship?;
    calc.get_item_attr_oextra(ctx, ship_uid, attr_rid)
        .map(PValue::from_value_clamped)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Time-limited processing (time-limited aggregators, or hard downtime processing)
////////////////////////////////////////////////////////////////////////////////////////////////////
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
impl<T> AggrPartDataTail<T>
where
    T: Copy,
{
    pub(super) fn get_duration_with_tail(&self) -> PValue {
        let mut duration = self.cycle_main_duration;
        if let Some(tail_duration) = self.cycle_tail_duration {
            duration += tail_duration;
        }
        duration
    }
}

pub(super) fn process_full_loop_lim_sin_with_cutoff<T, A>(
    accum: &mut A,
    p1_data: &AggrPartDataTail<T>,
    p1_repeat_count: Count,
    p2_data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
    loop_inner_duration: PValue,
    loop_repeat_count: Count,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Once hard downtime starts, instances cannot be applied
    let mut time = loop_inner_duration.into_value();
    let p1_full_repeat_count = p1_repeat_count.min(get_full_repeat_count(
        time,
        p1_data.cycle_main_duration,
        p1_data.cycle_tail_duration,
    ));
    let mut p1_remaining_repeat_count = p1_repeat_count;
    if p1_full_repeat_count > Count::ZERO {
        accum.add_instance(
            p1_data.output.get_instance(),
            chance_mult,
            p1_data.output.get_instance_count() * p1_full_repeat_count * loop_repeat_count,
        );
        time -= p1_data.cycle_main_duration * p1_full_repeat_count.into_pvalue();
        p1_remaining_repeat_count -= p1_full_repeat_count;
    }
    while p1_remaining_repeat_count > Count::ZERO {
        process_incomplete_cycle(accum, time, &p1_data.output, chance_mult, loop_repeat_count);
        time -= p1_data.cycle_main_duration;
        p1_remaining_repeat_count -= Count::ONE;
    }
    process_full_cycle_with_cutoff(accum, p2_data, chance_mult, loop_repeat_count);
}

pub(super) fn get_full_repeat_count(
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

pub(super) fn process_full_cycle_with_cutoff<T, A>(
    accum: &mut A,
    data: &AggrPartDataTail<T>,
    chance_mult: Option<PValue>,
    repeat_count: Count,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Hard downtimes cut output tails. If output has a tail (it couldn't be fit into main
    // duration), process cycle like partial
    match data.cycle_tail_duration.is_some() {
        true => process_incomplete_cycle(
            accum,
            data.cycle_main_duration.into_value(),
            &data.output,
            chance_mult,
            repeat_count,
        ),
        false => accum.add_instance(
            data.output.get_instance(),
            chance_mult,
            data.output.get_instance_count() * repeat_count,
        ),
    }
}

// Cheap processing works only when cycle + its tail (output / instance durations) fit into time;
// this function has more expensive, but more accurate processing for case when there is not enough
// time to fit everything
pub(super) fn process_incomplete_cycle<T, A>(
    accum: &mut A,
    mut time: Value,
    output: &Output<T>,
    chance_mult: Option<PValue>,
    repeat_count: Count,
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
        accum.add_instance(instance_data.instance, chance_mult, repeat_count)
    }
}
