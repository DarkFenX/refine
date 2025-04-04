use std::collections::HashMap;

use crate::{
    sol::{
        ItemGrpId, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValChargeGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeGroupChargeInfo>,
}

#[derive(Clone)]
pub struct ValChargeGroupChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Group ID of current charge.
    pub charge_group_id: ItemGrpId,
    /// Group IDs allowed by containing module.
    pub allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_fast(&mut self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValChargeGroupFail> {
        let mut charges = HashMap::new();
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail((charge_item_id, charge_info)) => {
                        if !kfs.contains(&charge_item_id) {
                            charges.insert(charge_item_id, charge_info.clone());
                        }
                        cache.fail((charge_item_id, charge_info));
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail((charge_item_id, charge_info)) => {
                    if !kfs.contains(charge_item_id) {
                        charges.insert(*charge_item_id, charge_info.clone());
                    }
                }
            }
        }
        if charges.is_empty() {
            return None;
        }
        Some(ValChargeGroupFail { charges })
    }
}

fn calculate_item_result(uad: &Uad, module_item_id: &ItemId) -> ValCache<(), (ItemId, ValChargeGroupChargeInfo)> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_item_id() {
        Some(charge_item_id) => charge_item_id,
        None => return ValCache::Pass(()),
    };
    let allowed_group_ids = module
        .get_a_extras()
        .unwrap()
        .charge_limit
        .as_ref()
        .unwrap()
        .group_ids
        .clone();
    let charge_group_id = match uad.items.get_item(&charge_item_id).unwrap().get_a_group_id() {
        Some(charge_group_id) => charge_group_id,
        None => return ValCache::Pass(()),
    };
    match allowed_group_ids.contains(&charge_group_id) {
        true => ValCache::Pass(()),
        false => ValCache::Fail((
            charge_item_id,
            ValChargeGroupChargeInfo {
                parent_item_id: *module_item_id,
                charge_group_id,
                allowed_group_ids,
            },
        )),
    }
}
