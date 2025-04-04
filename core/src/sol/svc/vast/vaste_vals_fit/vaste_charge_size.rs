use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::RSet,
};

#[derive(Copy, Clone)]
pub struct ValChargeSizeFail {
    pub parent_item_id: ItemId,
    pub charge_item_id: ItemId,
    pub charge_size: Option<AttrVal>,
    pub allowed_size: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_fast(&mut self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Vec<ValChargeSizeFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                ValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
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
    allowed_size: AttrVal,
) -> ValCache<AttrVal, ValChargeSizeFail> {
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
            return ValCache::Fail(ValChargeSizeFail {
                parent_item_id: *module_item_id,
                charge_item_id,
                charge_size: None,
                allowed_size,
            });
        }
    };
    match charge_size == allowed_size {
        true => ValCache::Pass(allowed_size),
        false => ValCache::Fail(ValChargeSizeFail {
            parent_item_id: *module_item_id,
            charge_item_id,
            charge_size: Some(charge_size),
            allowed_size,
        }),
    }
}
