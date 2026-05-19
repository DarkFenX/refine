use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared::{
        AggrHardDtNull, AggrHardDtSimple, AggrPartData, AggrPartDataTail, process_output_of_cycle_with_cutoff,
        process_output_of_lls_with_cutoff,
    },
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
};

// Local effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_local_looped<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<IA>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(cseq) = cseq.try_loop_cseq() else {
        return false;
    };
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    match cseq.get_hard_dt().is_some() {
        true => process_hard_dt(cseq.convert_with_and_optimize(&mut converter), accum),
        false => process_regular(cseq.convert_with_and_optimize(&mut converter), accum),
    }
    true
}

fn process_regular<I, IA>(cseq: CycleSeqLooped<AggrPartData<I>, AggrHardDtNull>, accum: &mut SeqAccum<IA>)
where
    I: Copy,
    IA: SeqInstanceAccum<I>,
{
    for cycle_part in cseq.iter_cseq_parts() {
        accum.add_output_full(&cycle_part.data.output, None, cycle_part.repeat_count);
        accum.time += cycle_part.data.cycle_main_duration * cycle_part.repeat_count.into_pvalue();
    }
}

fn process_hard_dt<I, IA>(cseq: CycleSeqLooped<AggrPartDataTail<I>, AggrHardDtSimple>, accum: &mut SeqAccum<IA>)
where
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
