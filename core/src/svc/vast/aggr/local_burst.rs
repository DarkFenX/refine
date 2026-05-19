use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared::{AggrHardDtNull, AggrPartData},
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Local effects, considers only first cycle (for "burst" stats)
// Hard downtime is ignored, since burst cseqs are supposed not to have it
#[must_use]
pub(in crate::svc::vast) fn aggr_local_burst<BG, BX, I, IA>(
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
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    process_regular(cseq.convert_with_and_optimize(&mut converter), accum);
    true
}

fn process_regular<I, IA>(cseq: CycleSeq<AggrPartData<I>, AggrHardDtNull>, accum: &mut SeqAccum<IA>)
where
    I: Copy,
    IA: SeqInstanceAccum<I>,
{
    let first_cycle = cseq.get_first_cycle();
    accum.add_output_full(&first_cycle.output, None, Count::ONE);
    accum.time += first_cycle.cycle_main_duration;
}
