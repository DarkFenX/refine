use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        Count, ItemKey,
        svc::EffectSpec,
        uad::{Uad, item::UadItem},
    },
    util::round,
};

pub(in crate::sol::svc::vast) enum EffectCharge {
    NoCharge,
    Charge(EffectChargeInfo),
}
impl EffectCharge {
    pub(in crate::sol::svc::vast) fn get_cycle_count(&self) -> Option<Count> {
        if let Self::Charge(charge_info) = self
            && let EffectChargeCountKind::Count(count_info) = charge_info.count_info
        {
            return Some(count_info.cycle_count);
        }
        None
    }
}

pub(in crate::sol::svc::vast) struct EffectChargeInfo {
    pub(in crate::sol::svc::vast) item_key: ItemKey,
    pub(in crate::sol::svc::vast) count_info: EffectChargeCountKind,
}

pub(in crate::sol::svc::vast) enum EffectChargeCountKind {
    Count(EffectChargeCountInfo),
    Infinite,
}

#[derive(Copy, Clone)]
pub(in crate::sol::svc::vast) struct EffectChargeCountInfo {
    pub(in crate::sol::svc::vast) charge_count: Count,
    pub(in crate::sol::svc::vast) cycle_count: Count,
}

pub(in crate::sol::svc::vast) fn get_effect_charge(uad: &Uad, espec: &EffectSpec) -> EffectCharge {
    match uad.src.get_a_effect(&espec.a_effect_id).unwrap().charge {
        Some(ad::AEffectChargeInfo::Loaded) => {
            let parent_item = uad.items.get(espec.item_key);
            let charge_item_key = match parent_item.get_charge_item_key() {
                Some(charge_item_key) => charge_item_key,
                None => return EffectCharge::NoCharge,
            };
            let charge_item = uad.items.get(charge_item_key);
            EffectCharge::Charge(EffectChargeInfo {
                item_key: charge_item_key,
                count_info: get_count_info_for_loaded_charge(parent_item, charge_item),
            })
        }
        Some(ad::AEffectChargeInfo::Attr(_)) => {
            let parent_item = uad.items.get(espec.item_key);
            let charge_item_key = match parent_item.get_autocharges() {
                Some(autocharges) => match autocharges.get(&espec.a_effect_id) {
                    Some(charge_item_key) => *charge_item_key,
                    None => return EffectCharge::NoCharge,
                },
                None => return EffectCharge::NoCharge,
            };
            // For now, assume all items are loaded
            let count_info = match parent_item.get_a_effect_datas().unwrap().get(&espec.a_effect_id) {
                Some(a_item_effect_data) => match a_item_effect_data.charge_count {
                    Some(charge_count) => EffectChargeCountKind::Count(EffectChargeCountInfo {
                        charge_count,
                        cycle_count: charge_count,
                    }),
                    None => EffectChargeCountKind::Infinite,
                },
                None => EffectChargeCountKind::Infinite,
            };
            EffectCharge::Charge(EffectChargeInfo {
                item_key: charge_item_key,
                count_info,
            })
        }
        None => EffectCharge::NoCharge,
    }
}

fn get_count_info_for_loaded_charge(parent_item: &UadItem, charge_item: &UadItem) -> EffectChargeCountKind {
    let parent_capacity = match parent_item.get_a_attr(&ac::attrs::CAPACITY) {
        Some(capacity) if capacity != OF(0.0) => capacity,
        // No capacity = zero capacity = zero charges
        _ => {
            return EffectChargeCountKind::Count(EffectChargeCountInfo {
                charge_count: 0,
                cycle_count: 0,
            });
        }
    };
    let charge_volume = match charge_item.get_a_attr(&ac::attrs::VOLUME) {
        Some(volume) if volume != OF(0.0) => volume,
        // No volume = zero volume = infinite charges
        _ => return EffectChargeCountKind::Infinite,
    };
    // Rounding is protection against cases like 2.3 / 0.1 = 22.999999999999996
    let charge_count = round(parent_capacity / charge_volume, 10).floor() as Count;
    let charges_per_cycle = match parent_item.get_a_attr(&ac::attrs::CHARGE_RATE) {
        Some(charge_rate) => charge_rate.round() as Count,
        None => 1,
    };
    // Here it's assumed that an effect can cycle only when it has enough charges into it. This is
    // not true for items like AAR, which can cycle for partial rep efficiency, but since the lib
    //
    let cycle_count = charge_count / charges_per_cycle;
    EffectChargeCountKind::Count(EffectChargeCountInfo {
        charge_count,
        cycle_count,
    })
}
