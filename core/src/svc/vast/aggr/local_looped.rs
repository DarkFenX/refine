use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_looped::{SplitAccums, alooped_route_for_limited_cseq_nonspool, alooped_route_for_looped_cseq_nonspool},
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
pub(in crate::svc::vast) fn aggr_local_looped<BG, BX, I, IA>(
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
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let cseq = cseq.split_lim_loop().looped?;
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs)?;
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    alooped_route_for_looped_cseq_nonspool(cseq, None, &mut accum, &mut converter);
    Some(accum)
}

// Local effects, puts data for non-looped part into one accumulator, and for looped part into
// another
pub(in crate::svc::vast) fn aggr_local_split<BG, BX, I, IAO, IAL>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    mut accum_loop: SeqAccum<IAO>,
    mut accum_lim: IAL,
) -> SplitAccums<IAO, IAL>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IAO: SeqInstanceAccum<I>,
    IAL: SeqInstanceAccum<I>,
{
    let mut accums = SplitAccums::new();
    let Some(inv_local) = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) else {
        return accums;
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq = cseq.split_lim_loop();
    if let Some(cseq_limited) = cseq.limited {
        alooped_route_for_limited_cseq_nonspool(
            cseq_limited,
            cseq.looped.as_ref(),
            None,
            &mut accum_lim,
            &mut converter,
        );
        accums.limited = Some(accum_lim);
    }
    if let Some(cseq_looped) = cseq.looped {
        alooped_route_for_looped_cseq_nonspool(cseq_looped, None, &mut accum_loop, &mut converter);
        accums.looped = Some(accum_loop);
    }
    accums
}
