use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_output, get_proj_output_spool},
    shared::calc_charge_mult,
    traits::LimitInstance,
};
use crate::{
    misc::InfCount,
    num::{Count, PValue},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Projected effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_clip<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let inv_proj = match AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid) {
        Some(inv_proj) => inv_proj,
        None => return false,
    };
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum),
        None => aggr_regular(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_spool<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let mut uninterrupted_cycles = Count::ZERO;
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    'part: for cycle_part in cycle_parts.iter() {
        let part_cycle_count = match cycle_part.repeat_count {
            InfCount::Count(part_cycle_count) => part_cycle_count,
            InfCount::Infinite => match cycle_part.data.interrupt {
                // Process 1 cycle if reload happens after every cycle in this part, even if cycles
                // are infinite
                Some(interrupt) if interrupt.reload => Count::ONE,
                // No reloads in infinite sequence - sequence is not a clip - no data to return
                _ => return false,
            },
        };
        // Calculate chargedness mult once for every part, no need to do it for every cycle
        let charge_mult = calc_charge_mult(ctx, calc, projector_uid, ospec.charge_mult, cycle_part.data.chargedness);
        for i in Count::ZERO..part_cycle_count {
            // Case when the rest of cycle part is at full spool
            if cycle_part.data.interrupt.is_none() && uninterrupted_cycles >= inv_spool.cycles_to_max {
                let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, inv_spool.max);
                let remaining_cycles = part_cycle_count - i;
                uninterrupted_cycles += remaining_cycles;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * remaining_cycles,
                );
                accum.time += cycle_part.data.duration * remaining_cycles.into_pvalue();
                // No interruptions in this branch, no need to do handle reload flag
                continue 'part;
            }
            let spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_output_spool(&inv_proj, charge_mult, spool);
            match cycle_part.data.interrupt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
            accum.add_instance(
                cycle_output.get_instance(),
                inv_proj.chance_mult,
                cycle_output.get_instance_count(),
            );
            accum.time += cycle_part.data.duration;
            // If reload happens after it, set reload flag and quit all the cycling - clip is
            // considered finished upon hitting reload
            if let Some(interrupt) = cycle_part.data.interrupt
                && interrupt.reload
            {
                reload = true;
                break 'part;
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}

fn aggr_regular<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: AggrProjInvData<T>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let mut reload = false;
    let cycle_parts = cseq.get_cseq_parts();
    for cycle_part in cycle_parts.iter() {
        let cycle_output = get_proj_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle_part.data.chargedness);
        // Update total values
        match cycle_part.data.interrupt {
            // Add first cycle after which there is a reload
            Some(interrupt) if interrupt.reload => {
                reload = true;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count(),
                );
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
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * part_cycle_count,
                );
                accum.time += cycle_part.data.duration * part_cycle_count.into_pvalue();
            }
        }
    }
    // If cycles are infinite and have no reload, return no data
    !cycle_parts.loops || reload
}
