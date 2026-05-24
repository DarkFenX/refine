use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverterRegular, process_infinite_spool, process_limited_spool,
        process_output_of_spooling_lls_with_cutoff, process_single_spool,
    },
    shared::{AggrHardDtSimple, AggrPartDataSpoolTail, get_tailed_cycle_full_repeat_count},
    shared_time::{get_cutoff_cycle_full_repeat_count, process_regular},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CSeqLoopLimSin, CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::LibConverter,
};

// Projected effects, aggregates total output by specified time
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_time<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    accum: &mut SeqAccum<IA>,
    time: PValue,
) -> bool
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_proj) = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)
    else {
        return false;
    };
    match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => process_spool(
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
        None => {
            let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
            let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
            process_regular(cseq_conv, inv_proj.chance_mult, &mut accum.instances, time);
        }
    }
    accum.time += time;
    true
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_spool<BG, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<I>,
    accum: &mut IA,
    ptime: PValue,
    inv_spool: AggrSpoolInvData,
) where
    BG: NEffectOutputGetter,
    I: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
    match cseq {
        CycleSeq::Lim(inner) => match inner.data.soft_dt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => process_regular(
                inner.convert_with(&mut converter).optimize(),
                inv_proj.chance_mult,
                accum,
                ptime,
            ),
            // Spool is considered
            false => {
                let mut time = ptime.into_value();
                let mut uninterrupted_cycles = Count::ZERO;
                process_limited_spool(
                    &inv_proj,
                    &inv_spool,
                    converter.lib_convert(inner.data),
                    accum,
                    &mut time,
                    &mut uninterrupted_cycles,
                    inner.repeat_count,
                );
            }
        },
        CycleSeq::Inf(inner) => match inner.data.soft_dt.is_some() || inner.hard_dt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => process_regular(
                inner.convert_with(&mut converter).optimize(),
                inv_proj.chance_mult,
                accum,
                ptime,
            ),
            // Spool is considered
            false => {
                let mut time = ptime.into_value();
                let mut uninterrupted_cycles = Count::ZERO;
                process_infinite_spool(
                    &inv_proj,
                    &inv_spool,
                    converter.lib_convert(inner.data),
                    accum,
                    &mut time,
                    &mut uninterrupted_cycles,
                );
            }
        },
        CycleSeq::LimInf(inner) => match inner.p1_data.soft_dt.is_some() && inner.p2_data.soft_dt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => process_regular(
                inner.convert_with(&mut converter).optimize(),
                inv_proj.chance_mult,
                accum,
                ptime,
            ),
            false => {
                let mut time = ptime.into_value();
                let mut uninterrupted_cycles = Count::ZERO;
                process_limited_spool(
                    &inv_proj,
                    &inv_spool,
                    converter.lib_convert(inner.p1_data),
                    accum,
                    &mut time,
                    &mut uninterrupted_cycles,
                    inner.p1_repeat_count,
                );
                process_infinite_spool(
                    &inv_proj,
                    &inv_spool,
                    converter.lib_convert(inner.p2_data),
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
                true => process_regular(
                    inner.convert_with(&mut converter).optimize(),
                    inv_proj.chance_mult,
                    accum,
                    ptime,
                ),
                false => {
                    let mut time = ptime.into_value();
                    let mut uninterrupted_cycles = Count::ZERO;
                    process_limited_spool(
                        &inv_proj,
                        &inv_spool,
                        converter.lib_convert(inner.p1_data),
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                        inner.p1_repeat_count,
                    );
                    process_single_spool(
                        &inv_proj,
                        &inv_spool,
                        converter.lib_convert(inner.p2_data),
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                    process_infinite_spool(
                        &inv_proj,
                        &inv_spool,
                        converter.lib_convert(inner.p3_data),
                        accum,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                }
            }
        }
        CycleSeq::LoopLimSin(inner) => {
            match inner.p1_data.soft_dt.is_some() && (inner.p2_data.soft_dt.is_some() || inner.hard_dt.is_some()) {
                // Non-spool handling for case when interruptions happen every cycle
                true => process_regular(
                    inner.convert_with(&mut converter).optimize(),
                    inv_proj.chance_mult,
                    accum,
                    ptime,
                ),
                false => match inner.hard_dt {
                    Some(_) => process_lls_spool_hard_dt(
                        &inner.convert_with(&mut converter),
                        &inv_proj,
                        &inv_spool,
                        accum,
                        ptime,
                    ),
                    None => process_lls_spool(&inner.convert_with(&mut converter), &inv_proj, &inv_spool, accum, ptime),
                },
            }
        }
    }
}

fn process_lls_spool<I, IA>(
    cseq: &CSeqLoopLimSin<AggrPartDataSpoolTail, AggrHardDtSimple>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    accum: &mut IA,
    ptime: PValue,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut time = ptime.into_value();
    let mut uninterrupted_cycles = Count::ZERO;
    while time >= Value::ZERO {
        let mut loop_accum = accum.copy_blank();
        let saved_uninterrupted_cycles = uninterrupted_cycles;
        process_limited_spool(
            inv_proj,
            inv_spool,
            cseq.p1_data,
            &mut loop_accum,
            &mut time,
            &mut uninterrupted_cycles,
            cseq.p1_repeat_count,
        );
        process_single_spool(
            inv_proj,
            inv_spool,
            cseq.p2_data,
            &mut loop_accum,
            &mut time,
            &mut uninterrupted_cycles,
        );
        accum.merge(&loop_accum, Count::ONE);
        // We detect if next loop result is going to be the same as previous one by tracking
        // uninterrupted cycle count. If they are the same, then output added by next loop should be
        // the same, provided there is enough time for full loop
        if uninterrupted_cycles == saved_uninterrupted_cycles && time >= Value::ZERO {
            let loop_main_duration = cseq.get_full_duration();
            let loop_tail_duration = cseq.p2_data.cycle_tail_duration;
            let full_repeat_count = get_tailed_cycle_full_repeat_count(time, loop_main_duration, loop_tail_duration);
            // Fast-forward by count of full repeating loops remaining time can fit
            if full_repeat_count > Count::ZERO {
                accum.merge(&loop_accum, full_repeat_count);
                time -= loop_main_duration * full_repeat_count.into_pvalue();
            }
        }
    }
}

fn process_lls_spool_hard_dt<I, IA>(
    cseq: &CSeqLoopLimSin<AggrPartDataSpoolTail, AggrHardDtSimple>,
    inv_proj: &AggrProjInvData<I>,
    inv_spool: &AggrSpoolInvData,
    accum: &mut IA,
    ptime: PValue,
) where
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let mut time = ptime.into_value();
    let loop_inner_duration = cseq.get_full_duration();
    let loop_full_duration = loop_inner_duration + cseq.hard_dt.unwrap().duration;
    let loop_full_repeat_count = get_cutoff_cycle_full_repeat_count(time, loop_inner_duration, loop_full_duration);
    // Process full cycles
    if loop_full_repeat_count > Count::ZERO {
        let mut inner_accum = accum.copy_blank();
        process_output_of_spooling_lls_with_cutoff(cseq, inv_proj, inv_spool, &mut inner_accum, loop_inner_duration);
        accum.merge(&inner_accum, loop_full_repeat_count);
        time -= loop_full_duration * loop_full_repeat_count.into_pvalue();
    }
    // Process partial cycle
    // Hard downtime resets uninterrupted cycles, so always start from 0
    let mut uninterrupted_cycles = Count::ZERO;
    process_limited_spool(
        inv_proj,
        inv_spool,
        cseq.p1_data,
        accum,
        &mut time,
        &mut uninterrupted_cycles,
        cseq.p1_repeat_count,
    );
    process_single_spool(
        inv_proj,
        inv_spool,
        cseq.p2_data,
        accum,
        &mut time,
        &mut uninterrupted_cycles,
    );
}
