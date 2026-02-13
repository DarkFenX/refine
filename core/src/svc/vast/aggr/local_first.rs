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

// Local effects, considers only first cycle (for "burst" stats)
#[must_use]
pub(in crate::svc::vast) fn aggr_local_first<T, A>(
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
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let cycle_data = cseq.get_first_cycle();
    let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_data.chargedness);
    accum.add_instance(cycle_output.get_instance(), None, cycle_output.get_instance_count());
    accum.time += cycle_data.duration;
    true
}
