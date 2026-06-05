use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_looped::{alooped_process_both_for_cseq_hard_dt, alooped_process_both_for_cseq_regular},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq, CycleSeqLimited, CycleSeqLooped},
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
        true => alooped_process_both_for_cseq_hard_dt(cseq.convert_with_and_optimize(&mut converter), None, accum),
        false => alooped_process_both_for_cseq_regular(cseq.convert_with_and_optimize(&mut converter), None, accum),
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
    if let Some(cseq_limited) = cseq.limited {
        match get_time_until_hard_dt_for_split(&cseq_limited, cseq.looped.as_ref()) {
            Some(time_until_hard_dt) => (),
            None => (),
        }
        accum_data = true;
    }
    if let Some(cseq_looped) = cseq.looped {
        let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
        match cseq_looped.get_hard_dt().is_some() {
            true => alooped_process_both_for_cseq_hard_dt(
                cseq_looped.convert_with_and_optimize(&mut converter),
                None,
                accum_loop,
            ),
            false => alooped_process_both_for_cseq_regular(
                cseq_looped.convert_with_and_optimize(&mut converter),
                None,
                accum_loop,
            ),
        }
        accum_data = true;
    }
    accum_data
}

fn get_time_until_hard_dt_for_split(
    cseq_limited: &CycleSeqLimited<CycleDataFull>,
    cseq_looped: Option<&CycleSeqLooped<CycleDataFull, CSeqHardDtFull>>,
) -> Option<PValue> {
    let cseq_loop = cseq_looped?;
    if cseq_loop.get_hard_dt().is_none() {
        return None;
    }
    Some(cseq_limited.get_main_duration() + cseq_loop.get_main_duration())
}
