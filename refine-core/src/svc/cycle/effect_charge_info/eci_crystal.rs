use crate::{
    misc::InfCount,
    nd::NEffectChargeDeplCrystal,
    num::{Count, PValue, UnitInterval, Value},
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
    let Some(charge_count) = module.get_charge_count(ctx.u_data) else {
        return InfCount::Count(Count::ZERO);
    };
    if charge_count == Count::ZERO {
        return InfCount::Count(Count::ZERO);
    }
    let charge_uid = module.get_charge_uid().unwrap();
    let charge_item = ctx.u_data.items.get(charge_uid);
    let charge_attrs = match charge_item.get_r_item_attr_data() {
        Some(riad) => &riad.attrs,
        // Charge is not loaded - can't use it
        None => return InfCount::Count(Count::ZERO),
    };
    if charge_attrs
        .get_opt(ctx.ac().crystals_get_damaged)
        .map(|v| !v.is_flag_set())
        .unwrap_or(true)
    {
        return InfCount::Infinite;
    }
    // Damage or chance of 0 or not defined - can cycle infinitely
    let dmg = match calc.get_item_oattr_oextra(ctx, charge_uid, ctx.ac().crystal_volatility_dmg) {
        Some(dmg) => match dmg < Value::FLOAT_TOLERANCE {
            true => return InfCount::Infinite,
            false => PValue::from_value_unchecked(dmg),
        },
        None => return InfCount::Infinite,
    };
    let chance = match calc.get_item_oattr_oextra(ctx, charge_uid, ctx.ac().crystal_volatility_chance) {
        Some(chance) => match chance < Value::FLOAT_TOLERANCE {
            true => return InfCount::Infinite,
            false => UnitInterval::from_value_clamped(chance),
        },
        None => return InfCount::Infinite,
    };
    let hp = match charge_attrs.get_opt(ctx.ac().hp) {
        Some(&hp) => PValue::from_value_clamped(hp),
        None => PValue::ZERO,
    };
    let procs_until_killed = (hp / dmg).ceil_unerr();
    let cycle_count_per_charge = Count::from_pvalue_trunced(procs_until_killed / chance.into_pvalue());
    InfCount::Count(charge_count * cycle_count_per_charge)
}
