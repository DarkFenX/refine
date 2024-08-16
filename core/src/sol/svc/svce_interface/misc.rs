use crate::{ad, defs::SolItemId, ec, sol::item::SolItem};

pub(super) fn is_effect_projectable(effect: &ad::AEffect) -> bool {
    effect.category == ec::effcats::TARGET || effect.category == ec::effcats::SYSTEM || effect.buff.is_some()
}

pub(super) fn get_effect_powered_charge_id(item: &SolItem, effect: &ad::AEffect) -> Option<SolItemId> {
    let charge_info = match effect.charge {
        Some(charge_info) => charge_info,
        None => return None,
    };
    if !charge_info.run_effects {
        return None;
    }
    match charge_info.location {
        ad::AEffectChargeLocation::Loaded => item.get_charge_id(),
        ad::AEffectChargeLocation::Attr(_) => match item.get_autocharges() {
            Some(autocharges) => autocharges.get(&effect.id).map(|v| *v),
            None => None,
        },
    }
}
