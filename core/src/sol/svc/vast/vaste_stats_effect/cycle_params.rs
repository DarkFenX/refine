use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{Count, svc::EffectSpec, uad::Uad},
    util::round,
};

// None means infinite
pub(in crate::sol::svc::vast) fn get_effect_cycles_until_reload(uad: &Uad, espec: &EffectSpec) -> Option<Count> {
    match uad.src.get_a_effect(&espec.a_effect_id).unwrap().charge {
        Some(ad::AEffectChargeInfo::Loaded) => {
            let parent_item = uad.items.get(espec.item_key);
            let parent_capacity = match parent_item.get_a_attr(&ac::attrs::CAPACITY) {
                Some(capacity) if capacity != OF(0.0) => capacity,
                // No capacity = zero capacity = zero charges
                _ => return Some(0),
            };
            let charge_item = match parent_item.get_charge_item_key() {
                Some(charge_item_key) => uad.items.get(charge_item_key),
                None => return Some(0),
            };
            let charge_volume = match charge_item.get_a_attr(&ac::attrs::VOLUME) {
                Some(volume) if volume != OF(0.0) => volume,
                _ => return Some(0),
            };
            // Protection against cases like 2.3 / 0.1 = 22
            Some(round(parent_capacity / charge_volume, 10).floor() as Count)
        }
        Some(ad::AEffectChargeInfo::Attr(_)) => uad
            .items
            .get(espec.item_key)
            .get_a_effect_datas()
            .unwrap()
            .get(&espec.a_effect_id)
            .and_then(|v| v.charge_count),
        None => Some(0),
    }
}
