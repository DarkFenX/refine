use ordered_float::Float;

use crate::{
    def::{Count, OF},
    nd::NEffectChargeDeplCrystal,
    svc::{SvcCtx, calc::Calc, cycle::effect_charge_info::EffectChargeInfo},
    ud::UModule,
    util::{FLOAT_TOLERANCE, InfCount, ceil_unerr, trunc_unerr},
};

pub(in crate::svc::cycle) fn get_eci_crystal(
    ctx: SvcCtx,
    calc: &mut Calc,
    module: &UModule,
    n_charge_crystal: NEffectChargeDeplCrystal,
) -> EffectChargeInfo {
    EffectChargeInfo {
        fully_charged: internal_cycle_count(ctx, calc, module),
        part_charged: None,
        can_run_uncharged: n_charge_crystal.can_run_uncharged,
    }
}

fn internal_cycle_count(ctx: SvcCtx, calc: &mut Calc, module: &UModule) -> InfCount {
    let charge_count = match module.get_charge_count(ctx.u_data) {
        Some(charge_count) => charge_count,
        None => return InfCount::Count(0),
    };
    if charge_count == 0 {
        return InfCount::Count(0);
    }
    let charge_key = module.get_charge_key().unwrap();
    let charge_item = ctx.u_data.items.get(charge_key);
    let charge_attrs = match charge_item.get_attrs() {
        Some(attrs) => attrs,
        // Charge is not loaded - can't use it
        None => return InfCount::Count(0),
    };
    let attr_consts = ctx.ac();
    if charge_attrs
        .get_opt(attr_consts.crystals_get_damaged)
        .map(|v| v.abs())
        .unwrap_or(OF(0.0))
        < FLOAT_TOLERANCE
    {
        return InfCount::Infinite;
    }
    // Damage or chance of 0 or not defined - can cycle infinitely
    let dmg = match calc.get_item_oattr_oextra(ctx, charge_key, attr_consts.crystal_volatility_dmg) {
        Some(OF(0.0)) => return InfCount::Infinite,
        Some(dmg) => dmg,
        None => return InfCount::Infinite,
    };
    let chance = match calc.get_item_oattr_oextra(ctx, charge_key, attr_consts.crystal_volatility_chance) {
        Some(OF(0.0)) => return InfCount::Infinite,
        Some(dmg) => dmg,
        None => return InfCount::Infinite,
    };
    let hp = charge_attrs.get_opt(attr_consts.hp).copied().unwrap_or(OF(0.0));
    let procs_until_killed = ceil_unerr(hp / dmg);
    let cycle_count_per_charge = trunc_unerr(procs_until_killed / chance).into_inner() as Count;
    InfCount::Count(charge_count * cycle_count_per_charge)
}
