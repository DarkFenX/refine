use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    shared::{
        AggrHardDtNull, AggrHardDtSimple, AggrPartData, AggrPartDataTail, process_output_of_cycle_with_cutoff,
        process_output_of_lls_with_cutoff,
    },
    traits::InstanceDuration,
};
use crate::{num::Count, svc::cycle::CycleSeqLooped};

pub(super) fn process_regular<I, IA>(cseq: CycleSeqLooped<AggrPartData<I>, AggrHardDtNull>, accum: &mut SeqAccum<IA>)
where
    I: Copy,
    IA: SeqInstanceAccum<I>,
{
    for cycle_part in cseq.iter_cseq_parts() {
        accum.add_output_full(&cycle_part.data.output, None, cycle_part.repeat_count);
        accum.time += cycle_part.data.cycle_main_duration * cycle_part.repeat_count.into_pvalue();
    }
}

pub(super) fn process_hard_dt<I, IA>(
    cseq: CycleSeqLooped<AggrPartDataTail<I>, AggrHardDtSimple>,
    accum: &mut SeqAccum<IA>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeqLooped::Inf(inner) => {
            process_output_of_cycle_with_cutoff(&mut accum.instances, &inner.data, None, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
        CycleSeqLooped::LoopLimSin(inner) => {
            process_output_of_lls_with_cutoff(&mut accum.instances, &inner, None, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
    }
}
