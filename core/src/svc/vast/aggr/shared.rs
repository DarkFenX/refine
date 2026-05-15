use super::{accum::SeqInstanceAccum, traits::InstanceDuration};
use crate::{
    num::{Count, PValue, Value},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqInf, CSeqLoopLimSin, CycleDataFull},
        output::Output,
    },
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

impl<T> CSeqInf<AggrPartDataTail<T>>
where
    T: Copy,
{
    pub(super) fn get_full_duration(&self) -> PValue {
        self.data.cycle_main_duration
    }
}

impl CSeqLoopLimSin<CycleDataFull> {
    pub(super) fn get_full_duration(&self) -> PValue {
        self.p1_data
            .get_main_duration()
            .mul_add(self.p1_repeat_count.into_pvalue(), self.p2_data.get_main_duration())
    }
    pub(super) fn get_full_duration_without_p2_soft_dt(&self) -> PValue {
        self.p1_data
            .get_main_duration()
            .mul_add(self.p1_repeat_count.into_pvalue(), self.p2_data.active.duration)
    }
}
impl<T> CSeqLoopLimSin<AggrPartDataTail<T>>
where
    T: Copy,
{
    pub(super) fn get_full_duration(&self) -> PValue {
        self.p1_data
            .cycle_main_duration
            .mul_add(self.p1_repeat_count.into_pvalue(), self.p2_data.cycle_main_duration)
    }
}

pub(super) fn get_cycle_tail_duration(
    cycle_main_duration: PValue,
    output_completion_duration: PValue,
) -> Option<PValue> {
    let tail_duration = output_completion_duration - cycle_main_duration;
    match tail_duration > Value::ZERO {
        true => Some(PValue::from_value_unchecked(tail_duration)),
        false => None,
    }
}

pub(super) fn get_tailed_cycle_full_repeat_count(
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

pub(super) fn process_output_of_lls_with_cutoff<T, A>(
    accum: &mut A,
    cseq: &CSeqLoopLimSin<AggrPartDataTail<T>>,
    chance_mult: Option<PValue>,
    loop_repeat_count: Count,
) where
    T: Copy + InstanceDuration,
    A: SeqInstanceAccum<T>,
{
    // Once hard downtime starts, instances cannot be applied
    let mut time = cseq.get_full_duration().into_value();
    let p1_full_repeat_count = cseq.p1_repeat_count.min(get_tailed_cycle_full_repeat_count(
        time,
        cseq.p1_data.cycle_main_duration,
        cseq.p1_data.cycle_tail_duration,
    ));
    let mut p1_remaining_repeat_count = cseq.p1_repeat_count;
    if p1_full_repeat_count > Count::ZERO {
        accum.add_output_full(
            &cseq.p1_data.output,
            chance_mult,
            p1_full_repeat_count * loop_repeat_count,
        );
        time -= cseq.p1_data.cycle_main_duration * p1_full_repeat_count.into_pvalue();
        p1_remaining_repeat_count -= p1_full_repeat_count;
    }
    while p1_remaining_repeat_count > Count::ZERO {
        accum.add_output_time_limited(&cseq.p1_data.output, chance_mult, loop_repeat_count, time);
        time -= cseq.p1_data.cycle_main_duration;
        p1_remaining_repeat_count -= Count::ONE;
    }
    process_output_of_cycle_with_cutoff(accum, &cseq.p2_data, chance_mult, loop_repeat_count);
}

pub(super) fn process_output_of_cycle_with_cutoff<T, A>(
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
        true => accum.add_output_time_limited(
            &data.output,
            chance_mult,
            repeat_count,
            data.cycle_main_duration.into_value(),
        ),
        false => accum.add_output_full(&data.output, chance_mult, repeat_count),
    }
}
