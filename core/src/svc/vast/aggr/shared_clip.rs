use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    shared::{AggrPartData, AggrPartDataTail, process_output_for_cycle_hard_dt, process_output_for_lls_cseq_hard_dt},
    traits::{InstanceDuration, InstanceLimit},
};
use crate::{
    misc::InfCount,
    num::{Count, PValue},
    svc::cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq, GetMainDuration},
    util::LibConverter,
};

pub(super) fn aclip_process_both_for_cseq_regular<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    chance_mult: Option<PValue>,
    mut accum: SeqAccum<IA>,
    mut converter: C,
) -> Option<SeqAccum<IA>>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>,
{
    let cseq_parts = cseq.get_cseq_parts();
    for cseq_part in cseq_parts.iter() {
        let cseq_part_data_conv = converter.lib_convert(cseq_part.data);
        // Add first cycle after which there is a reload. Here we assume every part has 1+ cycle
        // count, which is something cseq creating functions uphold
        if let Some(soft_dt) = cseq_part.data.soft_dt
            && soft_dt.reason.reload
        {
            accum.add_output_full(&cseq_part_data_conv.output, chance_mult, Count::ONE);
            // Record only active duration before reload, ignore soft downtime duration
            accum.time += cseq_part.data.active.duration;
            return Some(accum);
        }
        let part_cycle_count = match cseq_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            // If any cycle repeats infinitely without running out, then it does not run out of
            // "clip", no clip - no data
            InfCount::Infinite => return None,
        };
        accum.add_output_full(&cseq_part_data_conv.output, chance_mult, part_cycle_count);
        accum.time += cseq_part_data_conv.cycle_main_duration * part_cycle_count.into_pvalue();
    }
    match cseq_parts.loops {
        // If we went through all parts without reloads, and they loop, then there is no "clip"
        true => None,
        false => Some(accum),
    }
}

pub(super) fn aclip_process_both_for_cseq_hard_dt<I, IA, C>(
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    chance_mult: Option<PValue>,
    mut accum: SeqAccum<IA>,
    mut converter: C,
) -> Option<SeqAccum<IA>>
where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match cseq {
        // Infinite cycle with hard downtime on every cycle means we have just that cycle in clip
        CycleSeq::LoopSin(inner) => {
            let inner_data_conv = converter.lib_convert(inner.data);
            process_output_for_cycle_hard_dt(&mut accum.instances, &inner_data_conv, chance_mult, Count::ONE);
            // Record time until reload or hard downtime starts
            let p1_final_cycle_duration = match inner.data.soft_dt {
                Some(soft_dt) if soft_dt.reason.reload => inner.data.active.duration,
                _ => inner_data_conv.cycle_main_duration,
            };
            accum.time += p1_final_cycle_duration;
            Some(accum)
        }
        CycleSeq::LoopLimSin(inner) => {
            if let Some(soft_dt) = inner.p1_data.soft_dt
                && soft_dt.reason.reload
            {
                // Case when there is a reload right after first cycle
                let inner_p1_data_conv: AggrPartData<_> = converter.lib_convert(inner.p1_data);
                let loop_inner_duration = inner_p1_data_conv
                    .cycle_main_duration
                    .mul_add(inner.p1_repeat_count.into_pvalue(), inner.p2_data.get_main_duration());
                match inner_p1_data_conv.output.get_completion_duration() > loop_inner_duration {
                    true => accum.add_output_time_limited(
                        &inner_p1_data_conv.output,
                        chance_mult,
                        Count::ONE,
                        loop_inner_duration.into_value(),
                    ),
                    false => accum.add_output_full(&inner_p1_data_conv.output, chance_mult, Count::ONE),
                }
                // Stop counting time at reload, after active cycle is finished
                accum.time += inner.p1_data.active.duration;
            } else {
                // Case when all sequence cycles are allowed to run, possibly with reload after the
                // last cycle
                let inner_conv = inner.convert_with(&mut converter);
                process_output_for_lls_cseq_hard_dt(&mut accum.instances, &inner_conv, chance_mult, Count::ONE);
                // Record time until reload or hard downtime starts
                let p2_final_cycle_duration = match inner.p2_data.soft_dt {
                    Some(soft_dt) if soft_dt.reason.reload => inner.p2_data.active.duration,
                    _ => inner_conv.p2_data.cycle_main_duration,
                };
                accum.time += inner_conv
                    .p1_data
                    .cycle_main_duration
                    .mul_add(inner_conv.p1_repeat_count.into_pvalue(), p2_final_cycle_duration);
            }
            Some(accum)
        }
        // Other sequence types do not have hard downtime, so this should be unreachable
        _ => unreachable!(),
    }
}
