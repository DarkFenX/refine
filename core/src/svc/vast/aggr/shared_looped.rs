use super::{
    accum::{SeqAccum, SeqInstanceAccum, SeqInstanceAccumMax, SeqInstanceAccumStackMax},
    shared::{
        AggrHardDtNull, AggrHardDtSimple, AggrPartData, AggrPartDataTail, process_output_for_cycle_hard_dt,
        process_output_for_lls_cseq_hard_dt,
    },
    shared_time::{process_output_for_part_limited_regular, process_output_for_part_single_regular},
    traits::InstanceDuration,
};
use crate::{
    num::{Count, PValue},
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeqLimited, CycleSeqLooped},
    util::{LibConverter, LibMax},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Return value which contains both aggregators, and some methods to access it
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct SplitAccums<IAO, IAL> {
    pub(super) looped: Option<SeqAccum<IAO>>,
    pub(super) limited: Option<IAL>,
}
impl<IAO, IAL> SplitAccums<IAO, IAL> {
    pub(super) fn new() -> Self {
        Self {
            looped: None,
            limited: None,
        }
    }
}
impl<I> SplitAccums<SeqInstanceAccumStackMax<I>, SeqInstanceAccumMax<I>> {
    pub(in crate::svc::vast) fn get_per_second(&self) -> Option<I>
    where
        I: Copy + std::ops::Div<PValue, Output = I>,
    {
        self.looped.as_ref().map(|v| v.get_per_second())
    }
    pub(in crate::svc::vast) fn get_max(&self) -> Option<I>
    where
        I: Copy + LibMax,
    {
        match (self.looped.as_ref(), self.limited.as_ref()) {
            (None, None) => None,
            (None, Some(limited)) => Some(limited.max),
            (Some(looped), None) => Some(looped.instances.max),
            (Some(looped), Some(limited)) => Some(looped.instances.max.lib_max(limited.max)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Higher-level routers
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn alooped_route_for_limited_cseq_nonspool<I, IA, C>(
    cseq_limited: CycleSeqLimited<CycleDataFull>,
    cseq_looped: Option<&CycleSeqLooped<CycleDataFull, CSeqHardDtFull>>,
    chance_mult: Option<PValue>,
    accum: &mut IA,
    converter: &mut C,
) where
    I: Copy + Eq + InstanceDuration,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match get_time_until_hard_dt_for_split(&cseq_limited, cseq_looped) {
        Some(time_until_hard_dt) => alooped_process_output_for_limited_cseq_hard_dt(
            cseq_limited.convert_with_and_optimize(converter),
            chance_mult,
            time_until_hard_dt,
            accum,
        ),
        None => alooped_process_output_for_limited_cseq_regular(
            cseq_limited.convert_with_and_optimize(converter),
            chance_mult,
            accum,
        ),
    }
}

pub(super) fn alooped_route_for_looped_cseq_nonspool<I, IA, C>(
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    chance_mult: Option<PValue>,
    accum: &mut SeqAccum<IA>,
    converter: &mut C,
) where
    I: Copy + Eq + InstanceDuration,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match cseq.get_hard_dt().is_some() {
        true => {
            alooped_process_both_for_looped_cseq_hard_dt(cseq.convert_with_and_optimize(converter), chance_mult, accum)
        }
        false => {
            alooped_process_both_for_looped_cseq_regular(cseq.convert_with_and_optimize(converter), chance_mult, accum)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Looped part processing
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_process_both_for_looped_cseq_regular<I, IA>(
    cseq: CycleSeqLooped<AggrPartData<I>, AggrHardDtNull>,
    chance_mult: Option<PValue>,
    accum: &mut SeqAccum<IA>,
) where
    I: Copy,
    IA: SeqInstanceAccum<I>,
{
    for cseq_part in cseq.iter_cseq_parts() {
        accum.add_output_full(&cseq_part.data.output, chance_mult, cseq_part.repeat_count);
        accum.time += cseq_part.data.cycle_main_duration * cseq_part.repeat_count.into_pvalue();
    }
}

pub(super) fn alooped_process_both_for_looped_cseq_hard_dt<I, IA>(
    cseq: CycleSeqLooped<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
    accum: &mut SeqAccum<IA>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeqLooped::LoopSin(inner) => {
            process_output_for_cycle_hard_dt(&mut accum.instances, &inner.data, chance_mult, Count::ONE);
            accum.time += inner.get_main_duration() + inner.hard_dt.unwrap().duration;
        }
        CycleSeqLooped::LoopLimSin(inner) => {
            process_output_for_lls_cseq_hard_dt(&mut accum.instances, &inner, chance_mult, Count::ONE);
            accum.time += inner.get_main_duration() + inner.hard_dt.unwrap().duration;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Limited part processing
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_process_output_for_limited_cseq_regular<I, IA>(
    cseq: CycleSeqLimited<AggrPartData<I>>,
    chance_mult: Option<PValue>,
    accum: &mut IA,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeqLimited::Lim(inner) => {
            accum.add_output_full(&inner.data.output, chance_mult, inner.repeat_count);
        }
        CycleSeqLimited::LimSin(inner) => {
            accum.add_output_full(&inner.p1_data.output, chance_mult, inner.p1_repeat_count);
            accum.add_output_full(&inner.p2_data.output, chance_mult, Count::ONE);
        }
    }
}

fn alooped_process_output_for_limited_cseq_hard_dt<I, IA>(
    cseq: CycleSeqLimited<AggrPartDataTail<I>>,
    chance_mult: Option<PValue>,
    time_until_hard_dt: PValue,
    accum: &mut IA,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeqLimited::Lim(inner) => process_output_for_part_limited_regular(
            accum,
            &mut time_until_hard_dt.into_value(),
            &inner.data,
            inner.repeat_count,
            chance_mult,
        ),
        CycleSeqLimited::LimSin(inner) => {
            let mut time = time_until_hard_dt.into_value();
            process_output_for_part_limited_regular(
                accum,
                &mut time,
                &inner.p1_data,
                inner.p1_repeat_count,
                chance_mult,
            );
            process_output_for_part_single_regular(accum, &mut time, &inner.p2_data, chance_mult);
        }
    }
}

fn get_time_until_hard_dt_for_split(
    cseq_limited: &CycleSeqLimited<CycleDataFull>,
    cseq_looped: Option<&CycleSeqLooped<CycleDataFull, CSeqHardDtFull>>,
) -> Option<PValue> {
    let cseq_loop = cseq_looped?;
    cseq_loop.get_hard_dt()?;
    Some(cseq_limited.get_main_duration() + cseq_loop.get_main_duration())
}
