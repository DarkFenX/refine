use ordered_float::OrderedFloat as OF;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::HSet,
};

#[derive(Copy, Clone)]
pub struct ValChargeVolumeFail {
    pub parent_item_id: ItemId,
    pub charge_item_id: ItemId,
    pub charge_volume: AttrVal,
    pub max_volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_fast(&mut self, kfs: &HSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail) => {
                        let ret_fail = !kfs.contains(&fail.charge_item_id);
                        cache.fail(fail);
                        if ret_fail {
                            return false;
                        }
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail) => {
                    if !kfs.contains(&fail.charge_item_id) {
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
        kfs: &HSet<ItemId>,
        uad: &Uad,
    ) -> Vec<ValChargeVolumeFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail) => {
                        if !kfs.contains(&fail.charge_item_id) {
                            fails.push(fail);
                        }
                        cache.fail(fail);
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail) => {
                    if !kfs.contains(&fail.charge_item_id) {
                        fails.push(*fail)
                    }
                }
            }
        }
        fails
    }
}

fn calculate_item_result(
    uad: &Uad,
    module_item_id: &ItemId,
    charge_volume: AttrVal,
) -> ValCache<AttrVal, ValChargeVolumeFail> {
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
        false => ValCache::Fail(ValChargeVolumeFail {
            parent_item_id: *module_item_id,
            charge_item_id: module.get_charge_item_id().unwrap(),
            charge_volume,
            max_volume: module_capacity,
        }),
    }
}
