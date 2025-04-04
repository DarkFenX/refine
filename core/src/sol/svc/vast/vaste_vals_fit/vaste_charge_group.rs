use crate::{
    sol::{
        ItemGrpId, ItemId,
        svc::vast::{ValCache, VastFitData},
        uad::Uad,
    },
    util::RSet,
};

#[derive(Clone)]
pub struct ValChargeGroupFail {
    pub parent_item_id: ItemId,
    pub charge_item_id: ItemId,
    pub charge_group_id: Option<ItemGrpId>,
    pub allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_fast(&mut self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Vec<ValChargeGroupFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail) => {
                        if !kfs.contains(&fail.charge_item_id) {
                            fails.push(fail.clone());
                        }
                        cache.fail(fail);
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail) => {
                    if !kfs.contains(&fail.charge_item_id) {
                        fails.push(fail.clone())
                    }
                }
            }
        }
        fails
    }
}

fn calculate_item_result(uad: &Uad, module_item_id: &ItemId) -> ValCache<(), ValChargeGroupFail> {
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
        false => ValCache::Fail(ValChargeGroupFail {
            parent_item_id: *module_item_id,
            charge_item_id,
            charge_group_id: Some(charge_group_id),
            allowed_group_ids,
        }),
    }
}
