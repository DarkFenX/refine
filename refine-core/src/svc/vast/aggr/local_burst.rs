use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared::AggrPartData,
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
    util::LibConverter,
};

// Local effects, considers only first cycle (for "burst" stats)
// Hard downtime is ignored, since burst cseqs are supposed not to have it
pub(in crate::svc::vast) fn aggr_local_burst<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    mut accum: SeqAccum<IA>,
) -> Option<SeqAccum<IA>>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs)?;
    let &first_cycle = cseq.get_first_cycle();
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    process_both_for_cycle_regular(converter.lib_convert(first_cycle), &mut accum);
    Some(accum)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation details
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_both_for_cycle_regular<I, IA>(cycle_data: AggrPartData<I>, accum: &mut SeqAccum<IA>)
where
    I: Copy,
    IA: SeqInstanceAccum<I>,
{
    accum.add_output_full(&cycle_data.output, None, Count::ONE);
    accum.time += cycle_data.cycle_main_duration;
}
