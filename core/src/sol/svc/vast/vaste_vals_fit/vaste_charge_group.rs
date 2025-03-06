use crate::{
    defs::{EItemGrpId, SolItemId},
    sol::{
        svc::vast::{SolValCache, SolVastFitData},
        uad::SolUad,
    },
};

#[derive(Clone)]
pub struct SolValChargeGroupFail {
    pub parent_item_id: SolItemId,
    pub charge_item_id: SolItemId,
    pub charge_group_id: Option<EItemGrpId>,
    pub allowed_group_ids: Vec<EItemGrpId>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_fast(&mut self, uad: &SolUad) -> bool {
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                SolValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        uad: &SolUad,
    ) -> Vec<SolValChargeGroupFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                SolValCache::Todo(_) => match calculate_item_result(uad, module_item_id) {
                    SolValCache::Pass(pass) => cache.pass(pass),
                    SolValCache::Fail(fail) => {
                        fails.push(fail.clone());
                        cache.fail(fail);
                    }
                    _ => (),
                },
                SolValCache::Pass(_) => (),
                SolValCache::Fail(fail) => fails.push(fail.clone()),
            }
        }
        fails
    }
}

fn calculate_item_result(uad: &SolUad, module_item_id: &SolItemId) -> SolValCache<(), SolValChargeGroupFail> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_id() {
        Some(charge_item_id) => charge_item_id,
        None => return SolValCache::Pass(()),
    };
    let allowed_group_ids = module
        .get_a_extras()
        .unwrap()
        .charge_limit
        .as_ref()
        .unwrap()
        .group_ids
        .clone();
    let charge_group_id = match uad.items.get_item(&charge_item_id).unwrap().get_group_id() {
        Some(charge_group_id) => charge_group_id,
        None => {
            return SolValCache::Fail(SolValChargeGroupFail {
                parent_item_id: *module_item_id,
                charge_item_id,
                charge_group_id: None,
                allowed_group_ids,
            });
        }
    };
    match allowed_group_ids.contains(&charge_group_id) {
        true => SolValCache::Pass(()),
        false => SolValCache::Fail(SolValChargeGroupFail {
            parent_item_id: *module_item_id,
            charge_item_id,
            charge_group_id: Some(charge_group_id),
            allowed_group_ids,
        }),
    }
}
