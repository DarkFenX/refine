use super::{
    precalc::{AggrPartData, aggr_precalc_by_time, get_count_full_repeats},
    proj_shared::{AggrProjInvData, AggrSpoolInvData, get_proj_output, get_proj_output_spool},
    traits::LimitAmount,
};
use crate::{
    def::{AttrVal, Count, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::shared::calc_charge_mult,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemKey,
};

// Projected effects, aggregates total output by specified time
pub(in crate::svc) fn aggr_proj_time_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    time: AttrVal,
) -> Option<T>
where
    T: Default
        + Copy
        + Eq
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    match AggrSpoolInvData::try_make(ctx, calc, projector_key, effect, ospec) {
        Some(inv_spool) => aggr_total_spool(
            ctx,
            calc,
            projector_key,
            effect,
            cseq,
            ospec,
            projectee_key,
            time,
            inv_spool,
        ),
        None => aggr_total_regular(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, time),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-spool
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_total_regular<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    time: AttrVal,
) -> Option<T>
where
    T: Default
        + Copy
        + Eq
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    if time < OF(0.0) {
        return None;
    }
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let precalc = match cseq {
        CycleSeq::Lim(inner) => {
            let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::Inf(inner) => {
            let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.data.chargedness);
            inner.convert_extend(opc)
        }
        CycleSeq::LimInf(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
        CycleSeq::LimSinInf(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
            let p3_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p3_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc, p3_opc)
        }
        CycleSeq::LoopLimSin(inner) => {
            let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
            let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
            inner.convert_extend(p1_opc, p2_opc)
        }
    };
    Some(aggr_precalc_by_time(precalc, time))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_total_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    mut time: AttrVal,
    inv_spool: AggrSpoolInvData,
) -> Option<T>
where
    T: Default
        + Copy
        + Eq
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    if time < OF(0.0) {
        return None;
    }
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    match cseq {
        CycleSeq::Lim(inner) => {
            match inner.data.interrupt.is_some() {
                // Non-spool handling for case when interruptions happen every cycle
                true => {
                    let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.data.chargedness);
                    let precalc = inner.convert_extend(opc);
                    Some(aggr_precalc_by_time(precalc, time))
                }
                // Spool is considered
                false => None,
            }
        }
        CycleSeq::Inf(inner) => {
            match inner.data.interrupt.is_some() {
                // Non-spool handling for case when interruptions happen every cycle
                true => {
                    let opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.data.chargedness);
                    let precalc = inner.convert_extend(opc);
                    Some(aggr_precalc_by_time(precalc, time))
                }
                // Spool is considered
                false => {
                    let mut total_amount = T::default();
                    let mut uninterrupted_cycles = 0;
                    process_infinite_spool(
                        ctx,
                        calc,
                        projector_key,
                        ospec,
                        &inv_proj,
                        inv_spool,
                        inner.data,
                        &mut total_amount,
                        &mut time,
                        &mut uninterrupted_cycles,
                    );
                    Some(total_amount)
                }
            }
        }
        CycleSeq::LimInf(inner) => match inner.p1_data.interrupt.is_some() && inner.p2_data.interrupt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => {
                let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
                let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
                let precalc = inner.convert_extend(p1_opc, p2_opc);
                Some(aggr_precalc_by_time(precalc, time))
            }
            false => None,
        },
        CycleSeq::LimSinInf(inner) => match inner.p1_data.interrupt.is_some()
            && inner.p2_data.interrupt.is_some()
            && inner.p3_data.interrupt.is_some()
        {
            // Non-spool handling for case when interruptions happen every cycle
            true => {
                let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
                let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
                let p3_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p3_data.chargedness);
                let precalc = inner.convert_extend(p1_opc, p2_opc, p3_opc);
                Some(aggr_precalc_by_time(precalc, time))
            }
            false => None,
        },
        CycleSeq::LoopLimSin(inner) => match inner.p1_data.interrupt.is_some() && inner.p2_data.interrupt.is_some() {
            // Non-spool handling for case when interruptions happen every cycle
            true => {
                let p1_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p1_data.chargedness);
                let p2_opc = get_proj_output(ctx, calc, projector_key, ospec, &inv_proj, inner.p2_data.chargedness);
                let precalc = inner.convert_extend(p1_opc, p2_opc);
                Some(aggr_precalc_by_time(precalc, time))
            }
            false => None,
        },
    }
}

fn process_single_spool<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    if *time < OF(0.0) {
        return;
    }
    match *time >= data.time + data.tail_time {
        true => *total_amount += data.output.get_amount_sum(),
        false => *total_amount += data.output.get_amount_sum_by_time(*time),
    }
    *time -= data.time;
}

fn process_limited_spool<T>(total_amount: &mut T, time: &mut AttrVal, data: &AggrPartData<T>, repeat_count: Count)
where
    T: Default + Copy + std::ops::AddAssign<T> + std::ops::Mul<AttrVal, Output = T>,
{
    if *time < OF(0.0) {
        return;
    }
    let full_repeats = repeat_count.min(get_count_full_repeats(*time, data.time, data.tail_time).into_inner() as Count);
    *total_amount += data.output.get_amount_sum() * AttrVal::from(full_repeats);
    let mut remaining_repeats = repeat_count - full_repeats;
    while *time >= OF(0.0) && remaining_repeats > 0 {
        *total_amount += data.output.get_amount_sum_by_time(*time);
        *time -= data.time;
        remaining_repeats -= 1;
    }
}

fn process_infinite_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    ospec: &REffectProjOpcSpec<T>,
    inv_proj: &AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
    cycle_data: CycleDataFull,
    total_amount: &mut T,
    time: &mut AttrVal,
    uninterrupted_cycles: &mut Count,
) where
    T: Default
        + Copy
        + std::ops::AddAssign<T>
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + LimitAmount,
{
    if *time < OF(0.0) {
        return;
    }
    let charge_mult = calc_charge_mult(ctx, calc, projector_key, ospec.charge_mult, cycle_data.chargedness);
    let cycle_tail_time = (inv_proj.output.get_completion_time() - cycle_data.time).max(OF(0.0));
    let cycle_completion_time = cycle_data.time + cycle_tail_time;
    while *time >= OF(0.0) {
        let cycle_spool = inv_spool.calc_cycle_spool(*uninterrupted_cycles);
        let cycle_output = get_proj_output_spool(inv_proj, charge_mult, cycle_spool);
        match *time >= cycle_completion_time {
            true => *total_amount += cycle_output.get_amount_sum(),
            false => *total_amount += cycle_output.get_amount_sum_by_time(*time),
        }
        *time -= cycle_data.time;
    }
}
