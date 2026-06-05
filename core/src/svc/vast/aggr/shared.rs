use super::{accum::SeqInstanceAccum, traits::InstanceDuration};
use crate::{
    num::{Count, PValue, Value},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CSeqLoopLimSin, GetDuration, GetMainDuration},
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
// CSeq data container - simple
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::vast) struct AggrPartData<I> {
    // Active + soft downtime duration combined
    pub(in crate::svc::vast) cycle_main_duration: PValue,
    pub(in crate::svc::vast) output: Output<I>,
}
impl<I> GetMainDuration for AggrPartData<I> {
    fn get_main_duration(&self) -> PValue {
        self.cycle_main_duration
    }
}

#[derive(Copy, Clone)]
pub(super) struct AggrHardDtNull;
impl From<CSeqHardDtFull> for AggrHardDtNull {
    fn from(_hard_dt: CSeqHardDtFull) -> Self {
        Self {}
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// CSeq data container - time-limited processing (time-limited aggregators, or hard downtime
// processing)
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataTail<I> {
    // Cycle active duration + soft downtime duration
    pub(super) cycle_main_duration: PValue,
    // After main duration part is complete, it takes this duration to finish with output
    pub(super) cycle_tail_duration: Option<PValue>,
    pub(super) output: Output<I>,
}
impl<I> AggrPartDataTail<I> {
    pub(super) fn get_duration_with_tail(&self) -> PValue {
        let mut duration = self.cycle_main_duration;
        if let Some(tail_duration) = self.cycle_tail_duration {
            duration += tail_duration;
        }
        duration
    }
}
impl<I> GetMainDuration for AggrPartDataTail<I> {
    fn get_main_duration(&self) -> PValue {
        self.cycle_main_duration
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct AggrHardDtSimple {
    pub(in crate::svc::vast) duration: PValue,
}
impl From<CSeqHardDtFull> for AggrHardDtSimple {
    fn from(hard_dt: CSeqHardDtFull) -> Self {
        Self {
            duration: hard_dt.duration,
        }
    }
}
impl GetDuration for AggrHardDtSimple {
    fn get_duration(&self) -> PValue {
        self.duration
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// CSeq data container - spool
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataSpool {
    // Active + soft downtime duration combined
    pub(super) cycle_main_duration: PValue,
    pub(super) soft_dt: bool,
    // Includes both invariant str mult and part-specific str mult
    pub(super) str_mult: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// CSeq data container - spool + time-limited processing (time-limited aggregators, or hard downtime
// processing
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartDataSpoolTail {
    // Active + soft downtime duration combined
    pub(super) cycle_main_duration: PValue,
    // Active + soft downtime duration, or output completion duration, whichever is longer
    pub(super) cycle_completion_duration: Value,
    pub(super) cycle_tail_duration: Option<PValue>,
    pub(super) soft_dt: bool,
    // Includes both invariant str mult and part-specific str mult
    pub(super) str_mult: PValue,
}
impl GetMainDuration for AggrPartDataSpoolTail {
    fn get_main_duration(&self) -> PValue {
        self.cycle_main_duration
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
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

pub(super) fn process_output_cseq_lls_hard_dt<I, IA>(
    accum: &mut IA,
    cseq: &CSeqLoopLimSin<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
    loop_repeat_count: Count,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    // Once hard downtime starts, instances cannot be applied
    let mut time = cseq.get_main_duration().into_value();
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
    process_output_cycle_hard_dt(accum, &cseq.p2_data, chance_mult, loop_repeat_count);
}

pub(super) fn process_output_cycle_hard_dt<I, IA>(
    accum: &mut IA,
    data: &AggrPartDataTail<I>,
    chance_mult: Option<PValue>,
    repeat_count: Count,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
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
