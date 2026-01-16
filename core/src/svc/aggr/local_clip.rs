use super::{
    local_shared::{AggrLocalInvData, get_local_output},
    shared::AggrAmount,
    traits::LimitAmount,
};
use crate::{
    misc::InfCount,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UItemId,
};

// Local effects, considers only part of sequence until charges are out
pub(in crate::svc) fn aggr_local_clip_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrAmount<T>>
where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + LimitAmount,
{
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec)?;
    let mut total_amount = T::default();
    let mut total_time = PValue::ZERO;
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_local_output(ctx, calc, item_uid, ospec, &inv_local, cycle_part.data.chargedness);
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                total_amount += cycle_output.get_amount_sum();
                total_time += cycle_part.data.duration;
                break;
            }
            _ => {
                let part_cycle_count = match cycle_part.repeat_count {
                    InfCount::Count(part_cycle_count) => part_cycle_count.into_pvalue(),
                    // If any cycle repeats infinitely without running out, then it does not run out
                    // of "clip", no clip - no data
                    InfCount::Infinite => return None,
                };
                total_amount += cycle_output.get_amount_sum() * part_cycle_count;
                total_time += cycle_part.data.duration * part_cycle_count;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    if cycle_parts.loops && !reload {
        return None;
    }
    Some(AggrAmount {
        amount: total_amount,
        duration: total_time,
    })
}
