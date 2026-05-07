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
        cycle::{CycleDataFull, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
};

// Local effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_local_looped<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    match cseq {
        CycleSeqLooped::Inf(inner) if let Some(dt_hard) = inner.dt_hard => (),
        CycleSeqLooped::LoopLimSin(inner) if let Some(dt_hard) = inner.dt_hard => (),
        _ => process_regular(ctx, calc, item_uid, cseq, ospec, accum, inv_local),
    }
    true
}

fn process_regular<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
) where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    for cycle_part in cseq.iter_cseq_parts() {
        if cycle_part.repeat_count == Count::ZERO {
            continue;
        }
        let cycle_output = get_local_output(
            ctx,
            calc,
            item_uid,
            ospec,
            &inv_local,
            cycle_part.data.active.chargedness,
        );
        accum.add_instance(
            cycle_output.get_instance(),
            None,
            cycle_output.get_instance_count() * cycle_part.repeat_count,
        );
        accum.time += cycle_part.data.get_main_duration() * cycle_part.repeat_count.into_pvalue();
    }
}
