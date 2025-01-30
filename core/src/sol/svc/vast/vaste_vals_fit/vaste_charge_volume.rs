use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{
        svc::vast::{SolValCache, SolVastFitData},
        uad::SolUad,
    },
};

#[derive(Copy, Clone)]
pub struct SolChargeVolumeValFail {
    pub parent_item_id: SolItemId,
    pub charge_item_id: SolItemId,
    pub charge_volume: AttrVal,
    pub max_volume: AttrVal,
}
impl SolChargeVolumeValFail {
    fn new(parent_item_id: SolItemId, charge_item_id: SolItemId, charge_volume: AttrVal, max_volume: AttrVal) -> Self {
        Self {
            parent_item_id,
            charge_item_id,
            charge_volume,
            max_volume,
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_volume_fast(&mut self, uad: &SolUad) -> bool {
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                SolValCache::Todo(max_volume) => match calculate_item_result(uad, module_item_id, *max_volume) {
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
    pub(in crate::sol::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        uad: &SolUad,
    ) -> Vec<SolChargeVolumeValFail> {
        let mut fails = Vec::new();
        for (module_item_id, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                SolValCache::Todo(max_volume) => match calculate_item_result(uad, module_item_id, *max_volume) {
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
    max_volume: AttrVal,
) -> SolValCache<AttrVal, SolChargeVolumeValFail> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_id() {
        Some(charge_item_id) => charge_item_id,
        None => return SolValCache::Pass(max_volume),
    };
    let charge_attrs = match uad.items.get_item(&charge_item_id).unwrap().get_attrs() {
        Some(charge_attrs) => charge_attrs,
        None => return SolValCache::Pass(max_volume),
    };
    let charge_volume = match charge_attrs.get(&ec::attrs::VOLUME) {
        Some(charge_volume) => *charge_volume,
        None => return SolValCache::Pass(max_volume),
    };
    match charge_volume <= max_volume {
        true => SolValCache::Pass(max_volume),
        false => SolValCache::Fail(SolChargeVolumeValFail::new(
            *module_item_id,
            charge_item_id,
            charge_volume,
            max_volume,
        )),
    }
}
