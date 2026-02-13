use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, get_local_output},
    traits::LimitInstance,
};
use crate::{
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Local effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_local_looped<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<T>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_part.data.chargedness);
        accum.add_instance(
            cycle_output.get_instance(),
            None,
            cycle_output.get_instance_count() * cycle_part.repeat_count,
        );
        accum.time += cycle_part.data.duration * cycle_part.repeat_count.into_pvalue();
    }
    true
}
