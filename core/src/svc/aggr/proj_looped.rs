use super::{
    proj_inv_data::{ProjInvariantData, SpoolInvariantData},
    traits::Aggregable,
};
use crate::{
    def::{AttrVal, Count, OF},
    nd::NChargeMultGetter,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleDataTimeInt, CycleSeq, CycleSeqLooped},
    },
    ud::UItemKey,
};

// Projected effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_proj_looped_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let inv_spool = SpoolInvariantData::try_make(ctx, calc, projector_key, effect, ospec);
    // Spool needs interruption data, charge multiplier needs chargedness data
    match (inv_spool, ospec.charge_mult) {
        (Some(inv_spool), Some(charge_mult_getter)) => aggr_proj_chargedness(
            ctx,
            calc,
            projector_key,
            effect,
            cseq.into(),
            ospec,
            projectee_key,
            charge_mult_getter,
        ),
        (Some(inv_spool), None) => aggr_proj_spool(
            ctx,
            calc,
            projector_key,
            effect,
            cseq.into(),
            ospec,
            projectee_key,
            inv_spool,
        ),
        (None, Some(charge_mult_getter)) => aggr_proj_chargedness(
            ctx,
            calc,
            projector_key,
            effect,
            cseq.into(),
            ospec,
            projectee_key,
            charge_mult_getter,
        ),
        (None, None) => aggr_proj_basic(ctx, calc, projector_key, effect, cseq.into(), ospec, projectee_key),
    }
}

fn aggr_proj_spool<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeInt>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    inv_spool: SpoolInvariantData,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    // Do a dry run to set amount of interrupted cycles before we begin
    let mut uninterrupted_cycles = get_uninterrupted_cycles(&cseq, &inv_spool);
    let mut value = T::default();
    let mut time = OF(0.0);
    'part: for cycle_part in cseq.iter_cseq_parts() {
        for i in 0..cycle_part.repeat_count {
            let mut part_output = inv_proj.output;
            // Case when the rest of cycle part is at full spool
            if !cycle_part.data.interrupt && uninterrupted_cycles >= inv_spool.cycles_to_max {
                let remaining_cycles = cycle_part.repeat_count - i;
                // Spool
                part_output *= OF(1.0) + inv_spool.max;
                uninterrupted_cycles += remaining_cycles;
                // Limit
                if let Some(limit) = inv_proj.amount_limit {
                    part_output.limit_amount(limit);
                }
                // Update total values
                let remaining_cycles = AttrVal::from(remaining_cycles);
                value += part_output.instance_sum() * remaining_cycles;
                time += cycle_part.data.time * remaining_cycles;
                continue 'part;
            }
            // Spool
            part_output *= OF(1.0) + inv_spool.max.min(inv_spool.step * uninterrupted_cycles as f64);
            match cycle_part.data.interrupt {
                true => uninterrupted_cycles = 0,
                false => uninterrupted_cycles += 1,
            }
            // Limit
            if let Some(limit) = inv_proj.amount_limit {
                part_output.limit_amount(limit);
            }
            // Update total values
            value += part_output.instance_sum();
            time += cycle_part.data.time;
        }
    }
    Some(value / time)
}
fn get_uninterrupted_cycles(cseq: &CycleSeqLooped<CycleDataTimeInt>, inv_spool: &SpoolInvariantData) -> Count {
    let mut uninterrupted_cycles = 0;
    let mut interruptions = false;
    for cycle_part in cseq.iter_cseq_parts() {
        match cycle_part.data.interrupt {
            true => {
                uninterrupted_cycles = 0;
                interruptions = true;
            }
            false => {
                uninterrupted_cycles += cycle_part.repeat_count;
            }
        }
    }
    // If there are no interruptions at all, just set max possible spool right away
    if !interruptions {
        uninterrupted_cycles = inv_spool.cycles_to_max;
    }
    uninterrupted_cycles
}

fn aggr_proj_chargedness<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeCharge>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    charge_mult_getter: NChargeMultGetter,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut value = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_repeat_count = AttrVal::from(cycle_part.repeat_count);
        // Value
        let mut part_output = inv_proj.output;
        if let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
        {
            part_output *= charge_mult;
        }
        if let Some(limit) = inv_proj.amount_limit {
            part_output.limit_amount(limit);
        }
        value += part_output.instance_sum() * cycle_repeat_count;
        // Time
        time += cycle_part.data.time * cycle_repeat_count;
    }
    Some(value / time)
}

fn aggr_proj_basic<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTime>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut value = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_repeat_count = AttrVal::from(cycle_part.repeat_count);
        // Value
        let mut part_output = inv_proj.output;
        if let Some(limit) = inv_proj.amount_limit {
            part_output.limit_amount(limit);
        }
        value += part_output.instance_sum() * cycle_repeat_count;
        // Time
        time += cycle_part.data.time * cycle_repeat_count;
    }
    Some(value / time)
}
