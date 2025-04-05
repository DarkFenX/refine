use std::collections::HashMap;

use ordered_float::OrderedFloat as OF;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValChargeVolumeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeVolumeChargeInfo>,
}

#[derive(Copy, Clone)]
pub struct ValChargeVolumeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Volume of current charge.
    pub charge_volume: AttrVal,
    /// Maximum charge volume allowed by its parent module.
    pub max_volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_fast(&mut self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail((charge_item_id, charge_info)) => {
                        let ret_fail = !kfs.contains(&charge_item_id);
                        cache.fail((charge_item_id, charge_info));
                        if ret_fail {
                            return false;
                        }
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail((charge_item_id, _)) => {
                    if !kfs.contains(charge_item_id) {
                        return false;
                    }
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValChargeVolumeFail> {
        let mut charges = HashMap::new();
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail((charge_item_id, charge_info)) => {
                        if !kfs.contains(&charge_item_id) {
                            charges.insert(charge_item_id, charge_info);
                        }
                        cache.fail((charge_item_id, charge_info));
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail((charge_item_id, charge_info)) => {
                    if !kfs.contains(charge_item_id) {
                        charges.insert(*charge_item_id, *charge_info);
                    }
                }
            }
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeVolumeFail { charges }),
        }
    }
}

fn calculate_item_result(
    uad: &Uad,
    module_item_id: &ItemId,
    charge_volume: AttrVal,
) -> ValCache<AttrVal, (ItemId, ValChargeVolumeChargeInfo)> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let module_capacity = match module.get_a_attrs() {
        Some(attrs) => match attrs.get(&ac::attrs::CAPACITY) {
            Some(module_capacity) => *module_capacity,
            None => OF(0.0),
        },
        None => return ValCache::Pass(charge_volume),
    };
    match charge_volume <= module_capacity {
        true => ValCache::Pass(charge_volume),
        false => ValCache::Fail((
            module.get_charge_item_id().unwrap(),
            ValChargeVolumeChargeInfo {
                parent_item_id: *module_item_id,
                charge_volume,
                max_volume: module_capacity,
            },
        )),
    }
}
