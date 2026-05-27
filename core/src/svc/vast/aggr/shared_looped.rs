use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    shared::{
        AggrHardDtNull, AggrHardDtSimple, AggrPartData, AggrPartDataTail, process_output_of_cycle_with_cutoff,
        process_output_of_lls_with_cutoff,
    },
    traits::InstanceDuration,
};
use crate::{
    num::{Count, PValue},
    svc::cycle::CycleSeqLooped,
};

pub(super) fn process_regular<I, IA>(
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

pub(super) fn process_hard_dt<I, IA>(
    cseq: CycleSeqLooped<AggrPartDataTail<I>, AggrHardDtSimple>,
    chance_mult: Option<PValue>,
    accum: &mut SeqAccum<IA>,
) where
    I: Copy + InstanceDuration,
    IA: SeqInstanceAccum<I>,
{
    match cseq {
        CycleSeqLooped::LoopSin(inner) => {
            process_output_of_cycle_with_cutoff(&mut accum.instances, &inner.data, chance_mult, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
        CycleSeqLooped::LoopLimSin(inner) => {
            process_output_of_lls_with_cutoff(&mut accum.instances, &inner, chance_mult, Count::ONE);
            accum.time += inner.get_full_duration() + inner.hard_dt.unwrap().duration;
        }
    }
}
