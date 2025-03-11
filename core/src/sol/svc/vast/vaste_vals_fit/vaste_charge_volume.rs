use crate::{
    defs::{AttrVal, OF, SolItemId},
    ec,
    sol::{
        svc::vast::{SolValCache, SolVastFitData},
        uad::SolUad,
    },
    util::StSet,
};

#[derive(Copy, Clone)]
pub struct SolValChargeVolumeFail {
    pub parent_item_id: SolItemId,
    pub charge_item_id: SolItemId,
    pub charge_volume: AttrVal,
    pub max_volume: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_fast(
        &mut self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> bool {
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                SolValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    SolValCache::Pass(pass) => cache.pass(pass),
                    SolValCache::Fail(fail) => {
                        let ret_fail = !kfs.contains(&fail.charge_item_id);
                        cache.fail(fail);
                        if ret_fail {
                            return false;
                        }
                    }
                    _ => (),
                },
                SolValCache::Pass(_) => (),
                SolValCache::Fail(fail) => {
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
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> Vec<SolValChargeVolumeFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                SolValCache::Todo(charge_volume) => match calculate_item_result(uad, module_item_id, *charge_volume) {
                    SolValCache::Pass(pass) => cache.pass(pass),
                    SolValCache::Fail(fail) => {
                        if !kfs.contains(&fail.charge_item_id) {
                            fails.push(fail);
                        }
                        cache.fail(fail);
                    }
                    _ => (),
                },
                SolValCache::Pass(_) => (),
                SolValCache::Fail(fail) => {
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
    uad: &SolUad,
    module_item_id: &SolItemId,
    charge_volume: AttrVal,
) -> SolValCache<AttrVal, SolValChargeVolumeFail> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let module_capacity = match module.get_attrs() {
        Some(attrs) => match attrs.get(&ec::attrs::CAPACITY) {
            Some(module_capacity) => *module_capacity,
            None => OF(0.0),
        },
        // Policy is to pass validations if some data is not available due to item being not loaded
        None => return SolValCache::Pass(charge_volume),
    };
    match charge_volume <= module_capacity {
        true => SolValCache::Pass(charge_volume),
        false => SolValCache::Fail(SolValChargeVolumeFail {
            parent_item_id: *module_item_id,
            charge_item_id: module.get_charge_id().unwrap(),
            charge_volume,
            max_volume: module_capacity,
        }),
    }
}
