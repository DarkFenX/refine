use crate::{
    ac,
    def::{Count, OF},
    rd,
    svc::SvcCtx,
    ud::{UItem, UModule},
    util::{InfCount, trunc_unerr},
};

pub(super) fn get_autocharge_cycle_count(item: &UItem, effect: &rd::REffect) -> InfCount {
    let autocharges = match item.get_autocharges() {
        Some(autocharges) => autocharges,
        // Effect wants autocharge, but item does not support autocharges -> can't cycle
        None => return InfCount::Count(0),
    };
    if !autocharges.contains_ac_for_effect(&effect.get_key()) {
        // Effect wants autocharge, but no autocharge in the item - can't cycle. Since
        // autocharges are not add here when they cannot be loaded (no adapted item in
        // data source), non-loaded autocharges are covered by this as well.
        return InfCount::Count(0);
    };
    // Should always be available, since this method should never be requested for
    // non-loaded items
    let effect_datas = item.get_effect_datas().unwrap();
    match effect_datas.get(&effect.get_key()).unwrap().charge_count {
        Some(charge_count) => InfCount::Count(charge_count),
        None => InfCount::Infinite,
    }
}

pub(super) fn get_charge_rate_cycle_count(
    ctx: SvcCtx,
    module: &UModule,
    can_run_uncharged: bool,
    charged_optionals: bool,
) -> InfCount {
    if can_run_uncharged && !charged_optionals {
        return InfCount::Infinite;
    }
    let charge_count = match module.get_charge_count(ctx.u_data) {
        Some(charge_count) => charge_count,
        // When effect wants charge, but doesn't have one / it is not loaded - it can't cycle
        None => return InfCount::Count(0),
    };
    let charges_per_cycle = module.get_axt().unwrap().charge_rate;
    match charges_per_cycle == 0 {
        true => InfCount::Infinite,
        // Here it's assumed that an effect can cycle only when it has enough charges into it. This
        // is not true for items like AAR, which can cycle for partial rep efficiency, but since w/o
        // manual adjustments all AARs have enough paste to run w/o partial efficiency cycles, we
        // ignore this for simplicity's & performance's sake
        false => InfCount::Count(charge_count / charges_per_cycle),
    }
}

pub(super) fn get_crystal_cycle_count(
    ctx: SvcCtx,
    module: &UModule,
    can_run_uncharged: bool,
    charged_optionals: bool,
) -> InfCount {
    if can_run_uncharged && !charged_optionals {
        return InfCount::Infinite;
    }
    let charge_count = match module.get_charge_count(ctx.u_data) {
        // Not enough space to fit a single charge - can't cycle
        Some(0) => return InfCount::Count(0),
        Some(charge_count) => charge_count,
        // When effect wants charge, but doesn't have one / it is not loaded - can't cycle
        None => return InfCount::Count(0),
    };
    let charge_item = ctx.u_data.items.get(module.get_charge_key().unwrap());
    let charge_attrs = match charge_item.get_attrs() {
        Some(attrs) => attrs,
        // Charge is not loaded - can't cycle
        None => return InfCount::Count(0),
    };
    if charge_attrs
        .get(&ac::attrs::CRYSTALS_GET_DAMAGED)
        .copied()
        .unwrap_or(OF(0.0))
        == OF(0.0)
    {
        return InfCount::Infinite;
    }
    // Damage or chance of 0 or not defined - can cycle infinitely
    let dmg = match charge_attrs.get(&ac::attrs::CRYSTAL_VOLATILITY_DAMAGE) {
        Some(OF(0.0)) => return InfCount::Infinite,
        Some(dmg) => *dmg,
        None => return InfCount::Infinite,
    };
    let chance = match charge_attrs.get(&ac::attrs::CRYSTAL_VOLATILITY_CHANCE) {
        Some(OF(0.0)) => return InfCount::Infinite,
        Some(dmg) => *dmg,
        None => return InfCount::Infinite,
    };
    let hp = charge_attrs.get(&ac::attrs::HP).copied().unwrap_or(OF(0.0));
    let cycle_count_per_charge = trunc_unerr(hp / (dmg * chance)).into_inner() as Count;
    InfCount::Count(charge_count * cycle_count_per_charge)
}
