use crate::{
    nd::NChargeMultGetter,
    rd::REffectLocalOpcSpec,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleDataFull, CycleDataTime},
    },
    ud::UItemKey,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    cycle: &Cycle,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy,
{
    match ospec.charge_mult {
        Some(charge_mult_getter) => aggr_local_full(ctx, calc, item_key, cycle, ospec, charge_mult_getter),
        None => aggr_local_time(ctx, calc, item_key, cycle.into(), ospec),
    }
}

fn aggr_local_full<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    cycle: &Cycle<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<T>,
    charge_mult_getter: NChargeMultGetter,
) -> Option<T>
where
    T: Copy,
{
    let cycle_loop = cycle.try_get_loop()?;
    None
}

fn aggr_local_time<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    cycle: Cycle<CycleDataTime>,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy,
{
    let cycle_loop = cycle.try_get_loop()?;
    None
}
