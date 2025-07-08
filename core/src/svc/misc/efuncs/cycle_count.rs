use crate::{
    ac, ad,
    def::{Count, ItemKey},
    misc::{CycleCount, EffectSpec},
    nd::{NEffectCharge, NEffectChargeDepl},
    svc::SvcCtx,
};

pub(crate) fn get_espec_cycle_count(ctx: SvcCtx, espec: EffectSpec) -> CycleCount {
    let a_effect = ctx.uad.src.get_a_effect(&espec.a_effect_id).unwrap();
    get_effect_cycle_count(ctx, espec.item_key, a_effect)
}

pub(crate) fn get_effect_cycle_count(ctx: SvcCtx, item_key: ItemKey, a_effect: &ad::ArcEffectRt) -> CycleCount {
    match a_effect.hc.charge {
        Some(n_charge) => match n_charge {
            NEffectCharge::Autocharge(_) => get_autocharge_cycle_count(ctx, item_key, a_effect),
            NEffectCharge::Loaded(charge_depletion) => match charge_depletion {
                NEffectChargeDepl::ChargeRate => get_charge_rate_cycle_count(ctx, item_key),
                NEffectChargeDepl::Crystal => CycleCount::Infinite,
                NEffectChargeDepl::None => CycleCount::Infinite,
            },
        },
        None => CycleCount::Infinite,
    }
}

fn get_autocharge_cycle_count(ctx: SvcCtx, item_key: ItemKey, a_effect: &ad::ArcEffectRt) -> CycleCount {
    let uad_item = ctx.uad.items.get(item_key);
    let autocharges = match uad_item.get_autocharges() {
        Some(autocharges) => autocharges,
        // Effect wants autocharge, but item does not support autocharges -> can't cycle
        None => return CycleCount::Count(0),
    };
    if !autocharges.contains_ac_for_effect(&a_effect.ae.id) {
        // Effect wants autocharge, but no autocharge in the item - can't cycle. Since
        // autocharges are not add here when they cannot be loaded (no adapted item in
        // data source), non-loaded autocharges are covered by this as well.
        return CycleCount::Count(0);
    };
    // Should always be available, since this method should never be requested for
    // non-loaded items
    let a_effect_datas = uad_item.get_a_effect_datas().unwrap();
    match a_effect_datas.get(&a_effect.ae.id).unwrap().charge_count {
        Some(charge_count) => CycleCount::Count(charge_count),
        None => CycleCount::Infinite,
    }
}

fn get_charge_rate_cycle_count(ctx: SvcCtx, item_key: ItemKey) -> CycleCount {
    let uad_item = ctx.uad.items.get(item_key);
    let charge_count = match uad_item.get_charge_count(ctx.uad) {
        Some(charge_count) => charge_count,
        // When effect wants charge, but doesn't have one / it is not loaded - it can't cycle
        None => return CycleCount::Count(0),
    };
    let charges_per_cycle = match uad_item.get_a_attr(&ac::attrs::CHARGE_RATE) {
        Some(charge_rate) => charge_rate.round() as Count,
        None => 1,
    };
    // Here it's assumed that an effect can cycle only when it has enough charges into it. This is
    // not true for items like AAR, which can cycle for partial rep efficiency, but since w/o manual
    // adjustments all AARs have enough paste to run w/o partial efficiency cycles, we ignore this
    // for simplicity's & performance's sake
    CycleCount::Count(charge_count / charges_per_cycle)
}
