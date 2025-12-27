use super::{proj_inv_data::ProjInvariantData, traits::Aggregable};
use crate::{
    def::OF,
    nd::NChargeMultGetter,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
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
    match ospec.charge_mult {
        Some(charge_mult_getter) => aggr_proj_time_charge(
            ctx,
            calc,
            projector_key,
            effect,
            cseq.into(),
            ospec,
            projectee_key,
            charge_mult_getter,
        ),
        None => aggr_proj_time(ctx, calc, projector_key, effect, cseq.into(), ospec, projectee_key),
    }
}

fn aggr_proj_time_charge<T>(
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
        let cycle_repeat_count = OF::from(cycle_part.repeat_count);
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

fn aggr_proj_time<T>(
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
        let cycle_repeat_count = OF::from(cycle_part.repeat_count);
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
