use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared_time::process_single_regular,
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
        vast::aggr::traits::InstanceDuration,
    },
    ud::UItemId,
};

// Local effects, considers only first cycle (for "burst" stats)
#[must_use]
pub(in crate::svc::vast) fn aggr_local_first<BG, BX, T, A>(
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
    T: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let cycle_data = cseq.get_first_cycle();
    match cycle_data.dt_hard {
        // When there is hard downtime, limit output by pre-hard-downtime duration
        Some(dt_hard) => {
            let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
            let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
            let cycle_data_conv = cseq_conv.get_first_cycle();
            let mut time = cycle_data.get_main_duration().into_value();
            process_single_regular(&mut accum.instances, &mut time, cycle_data_conv, None);
            accum.time += dt_hard.duration;
        }
        None => {
            let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_data.active.chargedness);
            accum.add_instance(cycle_output.get_instance(), None, cycle_output.get_instance_count());
            accum.time += cycle_data.get_full_duration();
        }
    }
    true
}
