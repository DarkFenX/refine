use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter},
    shared_time::atime_process_output_for_cseq_regular,
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

// Local effects, aggregates total output by specified time
pub(in crate::svc::vast) fn aggr_local_time<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    mut accum: SeqAccum<IA>,
    time: PValue,
) -> Option<SeqAccum<IA>>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs)?;
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    atime_process_output_for_cseq_regular(
        cseq.convert_with_and_optimize(&mut converter),
        None,
        &mut accum.instances,
        time,
    );
    accum.time += time;
    Some(accum)
}
