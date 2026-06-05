use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_looped::{alooped_route_for_limited_cseq_nonspool, alooped_route_for_looped_cseq_nonspool},
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
    let Some(cseq) = cseq.split_lim_loop().looped else {
        return false;
    };
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    alooped_route_for_looped_cseq_nonspool(cseq, None, accum, &mut converter);
    true
}

// Local effects, puts data for non-looped part into one accumulator, and for looped part into
// another
#[must_use]
pub(in crate::svc::vast) fn aggr_local_split<BG, BX, I, IAL, IAO>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum_lim: &mut IAL,
    accum_loop: &mut SeqAccum<IAO>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IAL: SeqInstanceAccum<I>,
    IAO: SeqInstanceAccum<I>,
{
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return false;
    };
    let mut accum_data = false;
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq = cseq.split_lim_loop();
    if let Some(cseq_limited) = cseq.limited {
        alooped_route_for_limited_cseq_nonspool(cseq_limited, cseq.looped.as_ref(), None, accum_lim, &mut converter);
        accum_data = true;
    }
    if let Some(cseq_looped) = cseq.looped {
        alooped_route_for_looped_cseq_nonspool(cseq_looped, None, accum_loop, &mut converter);
        accum_data = true;
    }
    accum_data
}
