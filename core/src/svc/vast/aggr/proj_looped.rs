use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, get_proj_regular_output, get_proj_spool_cycle_output,
        get_proj_spool_part_str_mult,
    },
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq, CycleSeqLooped},
        output::Output,
    },
    ud::UItemId,
};

// Projected effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_looped<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_proj = match AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid) {
        Some(inv_proj) => inv_proj,
        None => return false,
    };
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool, accum),
        None => aggr_regular(ctx, calc, projector_uid, cseq, ospec, inv_proj, accum),
    }
}

fn aggr_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    for cycle_part in cseq.iter_cseq_parts() {
        if cycle_part.repeat_count == Count::ZERO {
            continue;
        }
        let cycle_output = get_proj_regular_output(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        accum.add_output_full(&cycle_output, inv_proj.chance_mult, cycle_part.repeat_count);
        accum.time += cycle_part.data.get_main_duration() * cycle_part.repeat_count.into_pvalue();
    }
    true
}

fn aggr_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    'part: for cycle_part in cseq.iter_cseq_parts() {
        // Part-specific strength mult
        let part_str_mult = get_proj_spool_part_str_mult(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_part.data.active.chargedness,
        );
        let mut prev_cycle_data: Option<CycleData<T>> = None;
        for i in Count::ZERO..cycle_part.repeat_count {
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            if let Some(prev_cycle_data) = prev_cycle_data
                && prev_cycle_data.spool == cycle_spool
            {
                let remaining_cycles = cycle_part.repeat_count - i;
                accum.add_output_full(&prev_cycle_data.output, inv_proj.chance_mult, remaining_cycles);
                accum.time += cycle_part.data.get_main_duration() * remaining_cycles.into_pvalue();
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_output = get_proj_spool_cycle_output(&inv_proj, part_str_mult, cycle_spool);
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            accum.time += cycle_part.data.get_main_duration();
            // Update state
            match cycle_part.data.soft_dt {
                Some(_) => uninterrupted_cycles = Count::ZERO,
                None => uninterrupted_cycles += Count::ONE,
            }
            prev_cycle_data = Some(CycleData {
                spool: cycle_spool,
                output: cycle_output,
            });
        }
    }
    true
}

struct CycleData<T>
where
    T: Copy,
{
    spool: Value,
    output: Output<T>,
}

fn get_uninterrupted_cycles(cseq: &CycleSeqLooped<CycleDataFull>, inv_spool: &AggrSpoolInvData) -> Count {
    let mut uninterrupted_cycles = Count::ZERO;
    if cseq.get_hard_dt().is_some() {
        return uninterrupted_cycles;
    }
    let mut downtimes = false;
    for cycle_part in cseq.iter_cseq_parts() {
        match cycle_part.data.soft_dt {
            Some(_) => {
                uninterrupted_cycles = Count::ZERO;
                downtimes = true;
            }
            None => {
                uninterrupted_cycles += cycle_part.repeat_count;
            }
        }
    }
    // If there are no interruptions at all, just set max possible spool right away
    if !downtimes {
        uninterrupted_cycles = inv_spool.cycles_to_max;
    }
    uninterrupted_cycles
}
