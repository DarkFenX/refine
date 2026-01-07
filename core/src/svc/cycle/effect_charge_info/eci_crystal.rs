use crate::{
    misc::{Count, InfCount, PValue, UnitInterval, Value},
    nd::NEffectChargeDeplCrystal,
    svc::{SvcCtx, calc::Calc, cycle::effect_charge_info::EffectChargeInfo},
    ud::UModule,
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
        None => return InfCount::Count(Count::ZERO),
    };
    if charge_count == Count::ZERO {
        return InfCount::Count(Count::ZERO);
    }
    let charge_uid = module.get_charge_uid().unwrap();
    let charge_item = ctx.u_data.items.get(charge_uid);
    let charge_attrs = match charge_item.get_attrs() {
        Some(attrs) => attrs,
        // Charge is not loaded - can't use it
        None => return InfCount::Count(Count::ZERO),
    };
    let attr_consts = ctx.ac();
    if charge_attrs
        .get_opt(attr_consts.crystals_get_damaged)
        .map(|v| v.abs())
        .unwrap_or(PValue::ZERO)
        < PValue::FLOAT_TOLERANCE
    {
        return InfCount::Infinite;
    }
    // Damage or chance of 0 or not defined - can cycle infinitely
    let dmg = match calc.get_item_oattr_oextra(ctx, charge_uid, attr_consts.crystal_volatility_dmg) {
        Some(dmg) => match dmg < Value::FLOAT_TOLERANCE {
            true => return InfCount::Infinite,
            false => PValue::from_val_unchecked(dmg),
        },
        None => return InfCount::Infinite,
    };
    let chance = match calc.get_item_oattr_oextra(ctx, charge_uid, attr_consts.crystal_volatility_chance) {
        Some(chance) => match chance < Value::FLOAT_TOLERANCE {
            true => return InfCount::Infinite,
            false => UnitInterval::from_value_clamped(chance),
        },
        None => return InfCount::Infinite,
    };
    let hp = match charge_attrs.get_opt(attr_consts.hp) {
        Some(&hp) => PValue::from_value_clamped(hp),
        None => PValue::ZERO,
    };
    let procs_until_killed = (hp / dmg).ceil_unerr();
    let cycle_count_per_charge = Count::from_pvalue_trunced(procs_until_killed / chance.into_pvalue());
    InfCount::Count(charge_count * cycle_count_per_charge)
}
