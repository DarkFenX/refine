use super::{local_inv_data::try_make_local_inv_data, traits::Aggregable};
use crate::{
    def::OF,
    nd::NChargeMultGetter,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleDataTime, CycleDataTimeChargedness},
    },
    ud::UItemKey,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cycle: &Cycle,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    match ospec.charge_mult {
        Some(charge_mult_getter) => {
            aggr_local_time_chargedness(ctx, calc, item_key, effect, cycle.into(), ospec, charge_mult_getter)
        }
        None => aggr_local_time(ctx, calc, item_key, effect, cycle.into(), ospec),
    }
}

fn aggr_local_time_chargedness<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cycle: Cycle<CycleDataTimeChargedness>,
    ospec: &REffectLocalOpcSpec<T>,
    charge_mult_getter: NChargeMultGetter,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cycle_loop = cycle.try_get_loop()?;
    let inv_data = try_make_local_inv_data(ctx, calc, item_key, effect, ospec)?;
    let mut output = T::default();
    let mut time = OF(0.0);
    for cycle_part in cycle_loop.iter_parts() {
        let mut part_output = inv_data.base.instance_sum() * OF::from(cycle_part.repeat_count);
        if let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            part_output *= charge_mult;
        }
        output += part_output;
        time += cycle_part;
    }
    None
}

fn aggr_local_time<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cycle: Cycle<CycleDataTime>,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cycle_loop = cycle.try_get_loop()?;
    let inv_data = try_make_local_inv_data(ctx, calc, item_key, effect, ospec)?;
    for cycle_part in cycle_loop.iter_parts() {}
    None
}
