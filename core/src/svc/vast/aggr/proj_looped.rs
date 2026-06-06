use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverter, get_proj_spool_cycle_output,
        process_output_for_lls_cseq_spool_hard_dt, process_output_for_part_limited_spool,
        process_output_for_part_single_spool,
    },
    shared::{
        AggrHardDtNull, AggrHardDtSimple, AggrPartData, AggrPartDataSpool, AggrPartDataSpoolTail, AggrPartDataTail,
    },
    shared_looped::{
        SplitAccums, alooped_process_both_for_looped_cseq_hard_dt, alooped_route_for_limited_cseq_nonspool,
        alooped_route_for_looped_cseq_nonspool, get_time_until_hard_dt_for_split,
    },
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CSeqLoopLimSin, CycleDataFull, CycleSeq, CycleSeqLimited, CycleSeqLooped},
    },
    ud::UItemId,
    util::LibConverter,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc::vast) fn aggr_proj_looped<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    mut accum: SeqAccum<IA>,
) -> Option<SeqAccum<IA>>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let cseq = cseq.split_lim_loop().looped?;
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)?;
    let inv_spool = AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec);
    let mut converter = ProjConverter::new(ctx, calc, projector_uid, ospec, &inv_proj);
    alooped_route_for_looped_cseq(cseq, inv_proj, inv_spool, &mut accum, &mut converter);
    Some(accum)
}

// Projected effects, puts data for non-looped part into one accumulator, and for looped part into
// another
pub(in crate::svc::vast) fn aggr_proj_split<BG, BX, I, IAO, IAL>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    mut accum_loop: SeqAccum<IAO>,
    mut accum_lim: IAL,
) -> SplitAccums<IAO, IAL>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IAO: SeqInstanceAccum<I>,
    IAL: SeqInstanceAccum<I>,
{
    let mut accums = SplitAccums::new();
    let Some(inv_proj) = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)
    else {
        return accums;
    };
    let inv_spool = AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec);
    let mut converter = ProjConverter::new(ctx, calc, projector_uid, ospec, &inv_proj);
    let cseq = cseq.split_lim_loop();
    if let Some(cseq_limited) = cseq.limited {
        alooped_route_for_limited_cseq(
            cseq_limited,
            cseq.looped.as_ref(),
            inv_proj,
            inv_spool,
            &mut accum_lim,
            &mut converter,
        );
        accums.limited = Some(accum_lim);
    }
    if let Some(cseq_looped) = cseq.looped {
        alooped_route_for_looped_cseq(cseq_looped, inv_proj, inv_spool, &mut accum_loop, &mut converter);
        accums.looped = Some(accum_loop);
    }
    accums
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Higher-level routers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_route_for_limited_cseq<I, IA, C>(
    cseq_limited: CycleSeqLimited<CycleDataFull>,
    cseq_looped: Option<&CycleSeqLooped<CycleDataFull, CSeqHardDtFull>>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: Option<AggrSpoolInvData>,
    accum: &mut IA,
    converter: &mut C,
) where
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>
        + LibConverter<CycleDataFull, AggrPartDataTail<I>>
        + LibConverter<CycleDataFull, AggrPartDataSpoolTail>,
{
    match inv_spool {
        Some(inv_spool) => match get_time_until_hard_dt_for_split(&cseq_limited, cseq_looped) {
            Some(time_until_hard_dt) => alooped_process_output_for_limited_cseq_spool_hard_dt(
                cseq_limited.convert_with_and_optimize(converter),
                inv_proj,
                inv_spool,
                time_until_hard_dt,
                accum,
            ),
            // TODO: implement
            None => (),
        },
        None => {
            alooped_route_for_limited_cseq_nonspool(cseq_limited, cseq_looped, inv_proj.chance_mult, accum, converter)
        }
    }
}

fn alooped_route_for_looped_cseq<I, IA, C>(
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: Option<AggrSpoolInvData>,
    accum: &mut SeqAccum<IA>,
    converter: &mut C,
) where
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartData<I>>
        + LibConverter<CycleDataFull, AggrPartDataTail<I>>
        + LibConverter<CycleDataFull, AggrPartDataSpool>
        + LibConverter<CycleDataFull, AggrPartDataSpoolTail>,
{
    match inv_spool {
        Some(inv_spool) => match cseq.get_hard_dt() {
            Some(_) => alooped_process_both_for_looped_cseq_spool_hard_dt(cseq, inv_proj, inv_spool, accum, converter),
            None => alooped_process_both_for_looped_cseq_spool(
                cseq.convert_with_and_optimize(converter),
                inv_proj,
                inv_spool,
                accum,
            ),
        },
        None => alooped_route_for_looped_cseq_nonspool(cseq, inv_proj.chance_mult, accum, converter),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific processing for looped part
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_process_both_for_looped_cseq_spool<I, IA>(
    cseq: CycleSeqLooped<AggrPartDataSpool, AggrHardDtNull>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_starting_uninterrupted_cycles(&cseq, &inv_spool);
    'part: for cseq_part in cseq.iter_cseq_parts() {
        for i in Count::ZERO..cseq_part.repeat_count {
            // Case when spool multiplier does not change for the rest of cycles of current part
            let stable_spool = match cseq_part.data.soft_dt {
                // Current cycle is at 0 spool, and we have an interrupt every cycle
                true if uninterrupted_cycles == Count::ZERO => Some(Value::ZERO),
                // Current cycle is at max spool, and we have no interrupts in cycles of current
                // part
                false if uninterrupted_cycles >= inv_spool.cycles_to_max => Some(inv_spool.max),
                _ => None,
            };
            if let Some(stable_spool) = stable_spool {
                let remaining_cycles = cseq_part.repeat_count - i;
                let cycle_output = get_proj_spool_cycle_output(&inv_proj, cseq_part.data.str_mult, stable_spool);
                accum.add_output_full(&cycle_output, inv_proj.chance_mult, remaining_cycles);
                accum.time += cseq_part.data.cycle_main_duration * remaining_cycles.into_pvalue();
                if !cseq_part.data.soft_dt {
                    uninterrupted_cycles += remaining_cycles;
                }
                // We've processed all the remaining cycles of current part, go next
                continue 'part;
            }
            let cycle_spool = inv_spool.calc_cycle_spool(uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(&inv_proj, cseq_part.data.str_mult, cycle_spool);
            accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
            accum.time += cseq_part.data.cycle_main_duration;
            // Update state
            match cseq_part.data.soft_dt {
                true => uninterrupted_cycles = Count::ZERO,
                false => uninterrupted_cycles += Count::ONE,
            }
        }
    }
}
fn get_starting_uninterrupted_cycles(
    cseq: &CycleSeqLooped<AggrPartDataSpool, AggrHardDtNull>,
    inv_spool: &AggrSpoolInvData,
) -> Count {
    let mut uninterrupted_cycles = Count::ZERO;
    if cseq.get_hard_dt().is_some() {
        return uninterrupted_cycles;
    }
    let mut downtimes = false;
    for cseq_part in cseq.iter_cseq_parts() {
        match cseq_part.data.soft_dt {
            true => {
                uninterrupted_cycles = Count::ZERO;
                downtimes = true;
            }
            false => {
                uninterrupted_cycles += cseq_part.repeat_count;
            }
        }
    }
    // If there are no interruptions at all, just set max possible spool right away
    if !downtimes {
        uninterrupted_cycles = inv_spool.cycles_to_max;
    }
    uninterrupted_cycles
}

fn alooped_process_both_for_looped_cseq_spool_hard_dt<I, IA, C>(
    cseq: CycleSeqLooped<CycleDataFull, CSeqHardDtFull>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    accum: &mut SeqAccum<IA>,
    converter: &mut C,
) where
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
    C: LibConverter<CycleDataFull, AggrPartDataTail<I>> + LibConverter<CycleDataFull, AggrPartDataSpoolTail>,
{
    // TODO: deoptimize looped vs non-looped here
    match cseq {
        // Infinite cycle with hard DT never spools up, process it the non-spool way
        CycleSeqLooped::LoopSin(_) => alooped_process_both_for_looped_cseq_hard_dt(
            cseq.convert_with_and_optimize(converter),
            inv_proj.chance_mult,
            accum,
        ),
        CycleSeqLooped::LoopLimSin(inner) => match inner.p1_data.soft_dt {
            // Composite loop with soft downtimes in first part and hard downtime after second also
            // does not spool up
            Some(_) => alooped_process_both_for_looped_cseq_hard_dt(
                cseq.convert_with_and_optimize(converter),
                inv_proj.chance_mult,
                accum,
            ),
            None => {
                let inner_conv: CSeqLoopLimSin<AggrPartDataSpoolTail, AggrHardDtSimple> = inner.convert_with(converter);
                let loop_inner_duration = inner_conv.get_main_duration();
                let loop_full_duration = loop_inner_duration + inner_conv.hard_dt.unwrap().duration;
                process_output_for_lls_cseq_spool_hard_dt(&inner_conv, &inv_proj, &inv_spool, &mut accum.instances);
                accum.time += loop_full_duration;
            }
        },
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific processing for limited part
////////////////////////////////////////////////////////////////////////////////////////////////////
fn alooped_process_output_for_limited_cseq_spool_hard_dt<I, IA>(
    cseq: CycleSeqLimited<AggrPartDataSpoolTail>,
    inv_proj: AggrProjInvData<I>,
    inv_spool: AggrSpoolInvData,
    time_until_hard_dt: PValue,
    accum: &mut IA,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut time = time_until_hard_dt.into_value();
    let mut uninterrupted_cycles = Count::ZERO;
    match cseq {
        CycleSeqLimited::Lim(inner) => process_output_for_part_limited_spool(
            &inv_proj,
            &inv_spool,
            inner.data,
            accum,
            &mut time,
            &mut uninterrupted_cycles,
            inner.repeat_count,
        ),
        CycleSeqLimited::LimSin(inner) => {
            process_output_for_part_limited_spool(
                &inv_proj,
                &inv_spool,
                inner.p1_data,
                accum,
                &mut time,
                &mut uninterrupted_cycles,
                inner.p1_repeat_count,
            );
            process_output_for_part_single_spool(
                &inv_proj,
                &inv_spool,
                inner.p2_data,
                accum,
                &mut time,
                &mut uninterrupted_cycles,
            );
        }
    }
}
