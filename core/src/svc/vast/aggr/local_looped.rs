use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared::{AggrPartData, AggrPartDataTail},
    shared_looped::{
        alooped_process_both_for_looped_cseq_hard_dt, alooped_process_both_for_looped_cseq_regular,
        alooped_process_output_for_limited_cseq_hard_dt, alooped_process_output_for_limited_cseq_regular,
        get_time_until_hard_dt_for_split,
    },
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
    util::LibConverter,
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
    alooped_process_both_for_looped_cseq(cseq, accum, &mut converter);
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
        alooped_process_both_for_limited_cseq(cseq_limited, cseq.looped.as_ref(), accum_lim, &mut converter);
        accum_data = true;
    }
    if let Some(cseq_looped) = cseq.looped {
        alooped_process_both_for_looped_cseq(cseq_looped, accum_loop, &mut converter);
        accum_data = true;
    }
    accum_data
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation details
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_process_both_for_limited_cseq<I, IA, C>(
    cseq_limited: CycleSeqLimited<CycleDataFull>,
    cseq_looped: Option<&CycleSeqLooped<CycleDataFull, CSeqHardDtFull>>,
    accum: &mut IA,
    converter: &mut C,
) where
    I: Copy + Eq + InstanceDuration,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match get_time_until_hard_dt_for_split(&cseq_limited, cseq_looped) {
        Some(time_until_hard_dt) => alooped_process_output_for_limited_cseq_hard_dt(
            cseq_limited.convert_with_and_optimize(converter),
            None,
            time_until_hard_dt,
            accum,
        ),
        None => alooped_process_output_for_limited_cseq_regular(
            cseq_limited.convert_with_and_optimize(converter),
            None,
            accum,
        ),
    }
}

fn alooped_process_both_for_looped_cseq<I, IA, C>(
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    accum: &mut SeqAccum<IA>,
    converter: &mut C,
) where
    I: Copy + Eq + InstanceDuration,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>> + LibConverter<CycleDataFull, AggrPartDataTail<I>>,
{
    match cseq.get_hard_dt().is_some() {
        true => alooped_process_both_for_looped_cseq_hard_dt(cseq.convert_with_and_optimize(converter), None, accum),
        false => alooped_process_both_for_looped_cseq_regular(cseq.convert_with_and_optimize(converter), None, accum),
    }
}
