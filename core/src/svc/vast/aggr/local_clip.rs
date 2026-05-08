use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, get_local_output},
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    misc::InfCount,
    nd::NEffectOutputGetter,
    num::{Count, PValue},
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
pub(in crate::svc::vast) fn aggr_local_clip<BG, BX, T, A>(
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
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    match cseq.get_hard_dt() {
        // Consider hard downtime as end of clip
        Some(hard_dt) => true,
        None => process_regular(ctx, calc, item_uid, cseq, ospec, accum, inv_local),
    }
}

fn process_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
) -> bool
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_local_output(
            ctx,
            calc,
            item_uid,
            ospec,
            &inv_local,
            cycle_part.data.active.chargedness,
        );
        match cycle_part.data.dt_soft {
            // Add first cycle after which there is a reload
            Some(soft_dt) if soft_dt.reason.reload => {
                reload = true;
                accum.add_instance(cycle_output.get_instance(), None, cycle_output.get_instance_count());
                accum.time += cycle_part.data.active.duration + soft_dt.duration;
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
                    accum.time += cycle_part.data.get_main_duration() * part_cycle_count.into_pvalue();
                }
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}
