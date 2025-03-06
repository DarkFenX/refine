use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{
        svc::vast::{SolValCache, SolVastFitData},
        uad::SolUad,
    },
};

#[derive(Copy, Clone)]
pub struct SolValChargeSizeFail {
    pub parent_item_id: SolItemId,
    pub charge_item_id: SolItemId,
    pub charge_size: Option<AttrVal>,
    pub allowed_size: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_fast(&mut self, uad: &SolUad) -> bool {
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                SolValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
                    SolValCache::Pass(pass) => cache.pass(pass),
                    SolValCache::Fail(fail) => {
                        cache.fail(fail);
                        return false;
                    }
                    _ => (),
                },
                SolValCache::Pass(_) => (),
                SolValCache::Fail(_) => return false,
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        uad: &SolUad,
    ) -> Vec<SolValChargeSizeFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_size.iter_mut() {
            match cache {
                SolValCache::Todo(allowed_size) => match calculate_item_result(uad, module_item_id, *allowed_size) {
                    SolValCache::Pass(pass) => cache.pass(pass),
                    SolValCache::Fail(fail) => {
                        fails.push(fail);
                        cache.fail(fail);
                    }
                    _ => (),
                },
                SolValCache::Pass(_) => (),
                SolValCache::Fail(fail) => fails.push(*fail),
            }
        }
        fails
    }
}

fn calculate_item_result(
    uad: &SolUad,
    module_item_id: &SolItemId,
    allowed_size: AttrVal,
) -> SolValCache<AttrVal, SolValChargeSizeFail> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_id() {
        Some(charge_item_id) => charge_item_id,
        None => return SolValCache::Pass(allowed_size),
    };
    let charge_attrs = match uad.items.get_item(&charge_item_id).unwrap().get_attrs() {
        Some(charge_attrs) => charge_attrs,
        None => {
            return SolValCache::Fail(SolValChargeSizeFail {
                parent_item_id: *module_item_id,
                charge_item_id,
                charge_size: None,
                allowed_size,
            });
        }
    };
    let charge_size = match charge_attrs.get(&ec::attrs::CHARGE_SIZE) {
        Some(charge_size) => *charge_size,
        None => {
            return SolValCache::Fail(SolValChargeSizeFail {
                parent_item_id: *module_item_id,
                charge_item_id,
                charge_size: None,
                allowed_size,
            });
        }
    };
    match charge_size == allowed_size {
        true => SolValCache::Pass(allowed_size),
        false => SolValCache::Fail(SolValChargeSizeFail {
            parent_item_id: *module_item_id,
            charge_item_id,
            charge_size: Some(charge_size),
            allowed_size,
        }),
    }
}
