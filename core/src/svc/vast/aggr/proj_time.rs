use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverterRegular, get_proj_spool_cycle_output,
        get_proj_spool_part_str_mult,
    },
    shared::get_full_repeat_count,
    shared_time::aggr_by_time,
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
};

// Projected effects, aggregates total output by specified time
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_time<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<A>,
    time: PValue,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_proj = match AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid) {
        Some(inv_proj) => inv_proj,
        None => return false,
    };
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_spool(
            ctx,
            calc,
            projector_uid,
            cseq,
            ospec,
            inv_proj,
            &mut accum.instances,
            time,
            inv_spool,
        ),
        None => aggr_regular(
            ctx,
            calc,
            projector_uid,
            cseq,
            ospec,
            inv_proj,
            &mut accum.instances,
            time,
        ),
    }
    accum.time += time;
    true
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-spool
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    accum: &mut A,
    time: PValue,
) where
    BG: NEffectOutputGetter,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, time);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    accum: &mut A,
    ptime: PValue,
    inv_spool: AggrSpoolInvData,
) where
    BG: NEffectOutputGetter,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    match cseq {
        CycleSeq::Lim(inner) => {
            match inner.data.soft_dt.is_some() {
                // Non-spool handling for case when interruptions happen every cycle
                true => {
                    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                    let cseq_conv = inner.convert_with(&mut converter).optimize();
                    aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, ptime);
                }
                // Spool is considered
                false => {
                    let mut time = ptime.into_value();
                    let mut uninterrupted_cycles = Count::ZERO;
                    process_limited_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.data,
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                        inner.repeat_count,
                    );
                }
            }
        }
        CycleSeq::Inf(inner) => {
            match inner.data.soft_dt.is_some() {
                // Non-spool handling for case when interruptions happen every cycle
                true => {
                    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                    let cseq_conv = inner.convert_with(&mut converter).optimize();
                    aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, ptime);
                }
                // Spool is considered
                false => {
                    let mut time = ptime.into_value();
                    let mut uninterrupted_cycles = Count::ZERO;
                    process_infinite_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.data,
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                }
            }
        }
        CycleSeq::LimInf(inner) => match inner.p1_data.soft_dt.is_some() && inner.p2_data.soft_dt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => {
                let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                let cseq_conv = inner.convert_with(&mut converter).optimize();
                aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, ptime);
            }
            false => {
                let mut time = ptime.into_value();
                let mut uninterrupted_cycles = Count::ZERO;
                process_limited_spool(
                    ctx,
                    calc,
                    projector_uid,
                    ospec,
                    &inv_proj,
                    &inv_spool,
                    inner.p1_data,
                    accum,
                    &mut time,
                    &mut uninterrupted_cycles,
                    inner.p1_repeat_count,
                );
                process_infinite_spool(
                    ctx,
                    calc,
                    projector_uid,
                    ospec,
                    &inv_proj,
                    &inv_spool,
                    inner.p2_data,
                    accum,
                    &mut time,
                    &mut uninterrupted_cycles,
                );
            }
        },
        CycleSeq::LimSinInf(inner) => {
            match inner.p1_data.soft_dt.is_some() && inner.p2_data.soft_dt.is_some() && inner.p3_data.soft_dt.is_some()
            {
                // Non-spool handling for case when interruptions happen every cycle
                true => {
                    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                    let cseq_conv = inner.convert_with(&mut converter).optimize();
                    aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, ptime);
                }
                false => {
                    let mut time = ptime.into_value();
                    let mut uninterrupted_cycles = Count::ZERO;
                    process_limited_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.p1_data,
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                        inner.p1_repeat_count,
                    );
                    process_single_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.p2_data,
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                    process_infinite_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.p3_data,
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                }
            }
        }
        CycleSeq::LoopLimSin(inner) => match inner.p1_data.soft_dt.is_some() && inner.p2_data.soft_dt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => {
                let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
                let cseq_conv = inner.convert_with(&mut converter).optimize();
                aggr_by_time(cseq_conv, inv_proj.chance_mult, accum, ptime)
            }
            false => {
                let mut time = ptime.into_value();
                let mut uninterrupted_cycles = Count::ZERO;
                while time >= Value::ZERO {
                    let mut loop_accum = accum.copy_blank();
                    let saved_interrupted_cycles = uninterrupted_cycles;
                    process_limited_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.p1_data,
                        &mut loop_accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                        inner.p1_repeat_count,
                    );
                    process_single_spool(
                        ctx,
                        calc,
                        projector_uid,
                        ospec,
                        &inv_proj,
                        &inv_spool,
                        inner.p2_data,
                        &mut loop_accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                    accum.merge(&loop_accum, Count::ONE);
                    // We detect if next loop result is going to be the same as previous one by
                    // tracking uninterrupted cycle count. If they are the same, then output added
                    // by next loop should be the same, provided there is enough time for full loop
                    if uninterrupted_cycles == saved_interrupted_cycles && time >= Value::ZERO {
                        let loop_duration = inner
                            .p1_data
                            .active_duration
                            .mul_add(inner.p1_repeat_count.into_pvalue(), inner.p2_data.active_duration);
                        let loop_tail_duration = PValue::from_value_clamped(
                            inv_proj.base_output.get_completion_duration() - inner.p2_data.active_duration,
                        );
                        let loop_full_repeat_count = get_full_repeat_count(time, loop_duration, loop_tail_duration);
                        // Fast-forward by count of full repeating loops remaining time can fit
                        if loop_full_repeat_count > Count::ZERO {
                            accum.merge(&loop_accum, loop_full_repeat_count);
                            time -= loop_duration * loop_full_repeat_count.into_pvalue();
                        }
                    }
                }
            }
        },
    }
}

fn process_single_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<T>,
    inv_spool: &AggrSpoolInvData,
    cycle_data: CycleDataFull,
    accum: &mut A,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
) where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    if *time < Value::ZERO {
        return;
    }
    let cycle_completion_duration = cycle_data
        .active_duration
        .max(inv_proj.base_output.get_completion_duration())
        .into_value();
    let part_str_mult = get_proj_spool_part_str_mult(ctx, calc, projector_uid, ospec, inv_proj, cycle_data.chargedness);
    let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
    let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, cycle_spool);
    match *time >= cycle_completion_duration {
        true => accum.add_instance(
            cycle_output.get_instance(),
            inv_proj.chance_mult,
            cycle_output.get_instance_count(),
        ),
        false => process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE),
    }
    *time -= cycle_data.active_duration;
    match cycle_data.interrupt {
        Some(_) => *uninterrupted_cycles = Count::ZERO,
        None => *uninterrupted_cycles += Count::ONE,
    }
}

fn process_limited_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<T>,
    inv_spool: &AggrSpoolInvData,
    cycle_data: CycleDataFull,
    accum: &mut A,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
    mut repeat_limit: Count,
) where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cycle_tail_duration =
        PValue::from_value_clamped(inv_proj.base_output.get_completion_duration() - cycle_data.active_duration);
    let cycle_completion_duration = (cycle_data.active_duration + cycle_tail_duration).into_value();
    let part_str_mult = get_proj_spool_part_str_mult(ctx, calc, projector_uid, ospec, inv_proj, cycle_data.chargedness);
    while *time >= Value::ZERO && repeat_limit > Count::ZERO {
        if cycle_data.interrupt.is_some() && *uninterrupted_cycles == Count::ZERO {
            // Shortcut #1: we're at 0 spool and can't spool for the rest of the sequence
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, Value::ZERO);
            let full_repeats = repeat_limit.min(get_full_repeat_count(
                *time,
                cycle_data.active_duration,
                cycle_tail_duration,
            ));
            // Full repeats
            if full_repeats > Count::ZERO {
                repeat_limit -= full_repeats;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * full_repeats,
                );
                *time -= cycle_data.active_duration * full_repeats.into_pvalue();
            }
            // Partial repeats
            while *time >= Value::ZERO && repeat_limit > Count::ZERO {
                repeat_limit -= Count::ONE;
                process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE);
                *time -= cycle_data.active_duration;
            }
            return;
        } else if cycle_data.interrupt.is_none() && *uninterrupted_cycles >= inv_spool.cycles_to_max {
            // Shortcut #2: we're at max spool and sequence is not interruptable
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, inv_spool.max);
            let full_repeats = repeat_limit.min(get_full_repeat_count(
                *time,
                cycle_data.active_duration,
                cycle_tail_duration,
            ));
            // Full repeats
            if full_repeats > Count::ZERO {
                repeat_limit -= full_repeats;
                *uninterrupted_cycles += full_repeats;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * full_repeats,
                );
                *time -= cycle_data.active_duration * full_repeats.into_pvalue();
            }
            // Partial repeats
            while *time >= Value::ZERO && repeat_limit > Count::ZERO {
                repeat_limit -= Count::ONE;
                *uninterrupted_cycles += Count::ONE;
                process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE);
                *time -= cycle_data.active_duration;
            }
            return;
        } else {
            let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, cycle_spool);
            match *time >= cycle_completion_duration {
                true => accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count(),
                ),
                false => process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE),
            }
            *time -= cycle_data.active_duration;
            match cycle_data.interrupt {
                Some(_) => *uninterrupted_cycles = Count::ZERO,
                None => *uninterrupted_cycles += Count::ONE,
            }
            repeat_limit -= Count::ONE;
        }
    }
}

fn process_infinite_spool<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: &AggrProjInvData<T>,
    inv_spool: &AggrSpoolInvData,
    cycle_data: CycleDataFull,
    accum: &mut A,
    time: &mut Value,
    uninterrupted_cycles: &mut Count,
) where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    if *time < Value::ZERO {
        return;
    }
    let cycle_tail_duration =
        PValue::from_value_clamped(inv_proj.base_output.get_completion_duration() - cycle_data.active_duration);
    let cycle_completion_duration = (cycle_data.active_duration + cycle_tail_duration).into_value();
    let part_str_mult = get_proj_spool_part_str_mult(ctx, calc, projector_uid, ospec, inv_proj, cycle_data.chargedness);
    while *time >= Value::ZERO {
        if cycle_data.interrupt.is_some() && *uninterrupted_cycles == Count::ZERO {
            // Shortcut #1: we're at 0 spool and can't spool for the rest of the sequence
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, Value::ZERO);
            let full_repeats = get_full_repeat_count(*time, cycle_data.active_duration, cycle_tail_duration);
            // Full repeats
            if full_repeats > Count::ZERO {
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * full_repeats,
                );
                *time -= cycle_data.active_duration * full_repeats.into_pvalue();
            }
            // Partial repeats
            while *time >= Value::ZERO {
                process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE);
                *time -= cycle_data.active_duration;
            }
            return;
        } else if cycle_data.interrupt.is_none() && *uninterrupted_cycles >= inv_spool.cycles_to_max {
            // Shortcut #2: we're at max spool and sequence is not interruptable
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, inv_spool.max);
            let full_repeats = get_full_repeat_count(*time, cycle_data.active_duration, cycle_tail_duration);
            // Full repeats
            if full_repeats > Count::ZERO {
                *uninterrupted_cycles += full_repeats;
                accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count() * full_repeats,
                );
                *time -= cycle_data.active_duration * full_repeats.into_pvalue();
            }
            // Partial repeats
            while *time >= Value::ZERO {
                *uninterrupted_cycles += Count::ONE;
                process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE);
                *time -= cycle_data.active_duration;
            }
            return;
        } else {
            // Regular cycle-by-cycle processing
            let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
            let cycle_output = get_proj_spool_cycle_output(inv_proj, part_str_mult, cycle_spool);
            match *time >= cycle_completion_duration {
                true => accum.add_instance(
                    cycle_output.get_instance(),
                    inv_proj.chance_mult,
                    cycle_output.get_instance_count(),
                ),
                false => process_output_time_limited(accum, *time, &cycle_output, inv_proj.chance_mult, Count::ONE),
            }
            *time -= cycle_data.active_duration;
            match cycle_data.interrupt {
                Some(_) => *uninterrupted_cycles = Count::ZERO,
                None => *uninterrupted_cycles += Count::ONE,
            }
        }
    }
}
