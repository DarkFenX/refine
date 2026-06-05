use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_looped::{process_hard_dt, process_regular},
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
    match cseq.get_hard_dt().is_some() {
        true => process_hard_dt(cseq.convert_with_and_optimize(&mut converter), None, accum),
        false => process_regular(cseq.convert_with_and_optimize(&mut converter), None, accum),
    }
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
    accum_lim: &mut SeqAccum<IAL>,
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
    let cseq = cseq.split_lim_loop();
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    if let Some(cseq_limited) = cseq.limited {
        accum_data = true;
    }
    if let Some(cseq_looped) = cseq.looped {
        match cseq_looped.get_hard_dt().is_some() {
            true => process_hard_dt(cseq_looped.convert_with_and_optimize(&mut converter), None, accum_loop),
            false => process_regular(cseq_looped.convert_with_and_optimize(&mut converter), None, accum_loop),
        }
        accum_data = true;
    }
    accum_data
}
