use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, get_local_output},
    traits::LimitInstance,
};
use crate::{
    Count,
    misc::InfCount,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Local effects, considers only part of sequence until charges are out
#[must_use]
pub(in crate::svc::vast) fn aggr_local_clip<T, BX, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<T, BX>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_part.data.chargedness);
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                accum.add_instance(cycle_output.get_instance(), None, cycle_output.get_instance_count());
                accum.time += cycle_part.data.duration;
                break;
            }
            _ => {
                let part_cycle_count = match cycle_part.repeat_count {
                    InfCount::Count(part_cycle_count) => part_cycle_count,
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return false,
                };
                if part_cycle_count > Count::ZERO {
                    accum.add_instance(
                        cycle_output.get_instance(),
                        None,
                        cycle_output.get_instance_count() * part_cycle_count,
                    );
                    accum.time += cycle_part.data.duration * part_cycle_count.into_pvalue();
                }
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}
