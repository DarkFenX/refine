use std::collections::HashMap;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValChargeSizeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeSizeChargeInfo>,
}

#[derive(Copy, Clone)]
pub struct ValChargeSizeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Size attribute value of current charge.
    pub charge_size: Option<AttrVal>,
    /// Size value allowed by module.
    pub allowed_size: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_fast(&mut self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = HashMap::new();
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
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
        if charges.is_empty() {
            return None;
        }
        Some(ValChargeSizeFail { charges })
    }
}

fn calculate_item_result(
    uad: &Uad,
    module_item_id: &ItemId,
    allowed_size: AttrVal,
) -> ValCache<AttrVal, (ItemId, ValChargeSizeChargeInfo)> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_item_id() {
        Some(charge_item_id) => charge_item_id,
        None => return ValCache::Pass(allowed_size),
    };
    let charge_attrs = match uad.items.get_item(&charge_item_id).unwrap().get_a_attrs() {
        Some(charge_attrs) => charge_attrs,
        None => return ValCache::Pass(allowed_size),
    };
    let charge_size = match charge_attrs.get(&ac::attrs::CHARGE_SIZE) {
        Some(charge_size) => *charge_size,
        None => {
            return ValCache::Fail((
                charge_item_id,
                ValChargeSizeChargeInfo {
                    parent_item_id: *module_item_id,
                    charge_size: None,
                    allowed_size,
                },
            ));
        }
    };
    match charge_size == allowed_size {
        true => ValCache::Pass(allowed_size),
        false => ValCache::Fail((
            charge_item_id,
            ValChargeSizeChargeInfo {
                parent_item_id: *module_item_id,
                charge_size: Some(charge_size),
                allowed_size,
            },
        )),
    }
}
