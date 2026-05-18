use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, get_local_output},
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
pub(in crate::svc::vast) fn aggr_local_burst<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let cycle_data = cseq.get_first_cycle();
    let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_data.active.chargedness);
    accum.add_output_full(&cycle_output, None, Count::ONE);
    accum.time += cycle_data.get_main_duration();
    true
}
