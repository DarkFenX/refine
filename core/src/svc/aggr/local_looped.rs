use super::{local_inv_data::LocalInvariantData, traits::Aggregable};
use crate::{
    def::OF,
    nd::NChargeMultGetter,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
    },
    ud::UItemKey,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    match ospec.charge_mult {
        Some(charge_mult_getter) => {
            aggr_local_time_charge(ctx, calc, item_key, effect, cseq.into(), ospec, charge_mult_getter)
        }
        None => aggr_local_time(ctx, calc, item_key, effect, cseq.into(), ospec),
    }
}

fn aggr_local_time_charge<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeCharge>,
    ospec: &REffectLocalOpcSpec<T>,
    charge_mult_getter: NChargeMultGetter,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_local = LocalInvariantData::try_make(ctx, calc, item_key, effect, ospec)?;
    let mut value = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_repeat_count = OF::from(cycle_part.repeat_count);
        // Value
        let mut part_output = inv_local.output;
        if let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            part_output *= charge_mult;
        }
        if let Some(limit) = inv_local.amount_limit {
            part_output.limit_amount(limit);
        }
        value += part_output.instance_sum() * cycle_repeat_count;
        // Time
        time += cycle_part.data.time * cycle_repeat_count;
    }
    Some(value / time)
}

fn aggr_local_time<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTime>,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_local = LocalInvariantData::try_make(ctx, calc, item_key, effect, ospec)?;
    let mut value = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let cycle_repeat_count = OF::from(cycle_part.repeat_count);
        // Value
        let mut part_output = inv_local.output;
        if let Some(limit) = inv_local.amount_limit {
            part_output.limit_amount(limit);
        }
        value += part_output.instance_sum() * cycle_repeat_count;
        // Time
        time += cycle_part.data.time * cycle_repeat_count;
    }
    Some(value / time)
}
