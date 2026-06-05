use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_clip::{aclip_process_both_for_cseq_hard_dt, aclip_process_both_for_cseq_regular},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Local effects, considers only part of sequence until charges are out
#[must_use]
pub(in crate::svc::vast) fn aggr_local_clip<BG, BX, I, IA>(
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
    I: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    match cseq.get_hard_dt().is_some() {
        true => aclip_process_both_for_cseq_hard_dt(cseq, None, accum, converter),
        false => aclip_process_both_for_cseq_regular(cseq, None, accum, converter),
    }
}
