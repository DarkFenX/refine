use crate::{
    ac, ad,
    def::{Count, ItemKey, OF},
    misc::EffectSpec,
    nd::{NEffectCharge, NEffectChargeDepl},
    svc::SvcCtx,
    util::{InfCount, trunc_unerr},
};

pub(in crate::svc) fn get_espec_cycle_count(ctx: SvcCtx, espec: EffectSpec) -> Option<InfCount> {
    let a_effect = ctx.uad.src.get_a_effect(&espec.a_effect_id)?;
    Some(get_effect_cycle_count(ctx, espec.item_key, a_effect))
}

pub(in crate::svc) fn get_effect_cycle_count(ctx: SvcCtx, item_key: ItemKey, a_effect: &ad::AEffectRt) -> InfCount {
    match a_effect.hc.charge {
        Some(n_charge) => match n_charge {
            NEffectCharge::Autocharge(_) => get_autocharge_cycle_count(ctx, item_key, a_effect),
            NEffectCharge::Loaded(charge_depletion) => match charge_depletion {
                NEffectChargeDepl::ChargeRate => get_charge_rate_cycle_count(ctx, item_key),
                NEffectChargeDepl::Crystal => get_crystal_cycle_count(ctx, item_key),
                NEffectChargeDepl::None => InfCount::Infinite,
            },
        },
        None => InfCount::Infinite,
    }
}

fn get_autocharge_cycle_count(ctx: SvcCtx, item_key: ItemKey, a_effect: &ad::AEffectRt) -> InfCount {
    let uad_item = ctx.uad.items.get(item_key);
    let autocharges = match uad_item.get_autocharges() {
        Some(autocharges) => autocharges,
        // Effect wants autocharge, but item does not support autocharges -> can't cycle
        None => return InfCount::Count(0),
    };
    if !autocharges.contains_ac_for_effect(&a_effect.ae.id) {
        // Effect wants autocharge, but no autocharge in the item - can't cycle. Since
        // autocharges are not add here when they cannot be loaded (no adapted item in
        // data source), non-loaded autocharges are covered by this as well.
        return InfCount::Count(0);
    };
    // Should always be available, since this method should never be requested for
    // non-loaded items
    let a_effect_datas = uad_item.get_a_effect_datas().unwrap();
    match a_effect_datas.get(&a_effect.ae.id).unwrap().charge_count {
        Some(charge_count) => InfCount::Count(charge_count),
        None => InfCount::Infinite,
    }
}

fn get_charge_rate_cycle_count(ctx: SvcCtx, item_key: ItemKey) -> InfCount {
    let uad_item = ctx.uad.items.get(item_key);
    let charge_count = match uad_item.get_charge_count(ctx.uad) {
        Some(charge_count) => charge_count,
        // When effect wants charge, but doesn't have one / it is not loaded - it can't cycle
        None => return InfCount::Count(0),
    };
    let charges_per_cycle = uad_item.get_a_xt().unwrap().charge_rate;
    match charges_per_cycle == 0 {
        true => InfCount::Infinite,
        // Here it's assumed that an effect can cycle only when it has enough charges into it. This
        // is not true for items like AAR, which can cycle for partial rep efficiency, but since w/o
        // manual adjustments all AARs have enough paste to run w/o partial efficiency cycles, we
        // ignore this for simplicity's & performance's sake
        false => InfCount::Count(charge_count / charges_per_cycle),
    }
}

fn get_crystal_cycle_count(ctx: SvcCtx, item_key: ItemKey) -> InfCount {
    let uad_item = ctx.uad.items.get(item_key);
    let charge_count = match uad_item.get_charge_count(ctx.uad) {
        // Not enough space to fit a single charge - can't cycle
        Some(0) => return InfCount::Count(0),
        Some(charge_count) => charge_count,
        // When effect wants charge, but doesn't have one / it is not loaded - can't cycle
        None => return InfCount::Count(0),
    };
    let charge_uad_item = ctx.uad.items.get(uad_item.get_charge_key().unwrap());
    let charge_attrs = match charge_uad_item.get_a_attrs() {
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
    let cycle_count_per_charge = trunc_unerr(hp / (dmg * chance)) as Count;
    InfCount::Count(charge_count * cycle_count_per_charge)
}
