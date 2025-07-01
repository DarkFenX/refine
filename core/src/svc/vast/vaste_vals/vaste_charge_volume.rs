use std::collections::HashMap;

use crate::{
    ac,
    def::{AttrVal, ItemId, ItemKey, OF},
    svc::{
        SvcCtx,
        vast::{ValCache, VastFitData},
    },
    util::RSet,
};

pub struct ValChargeVolumeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeVolumeChargeInfo>,
}

pub struct ValChargeVolumeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Volume of current charge.
    pub charge_volume: AttrVal,
    /// Maximum charge volume allowed by its parent module.
    pub max_volume: AttrVal,
}
impl ValChargeVolumeChargeInfo {
    fn from_fail_cache(ctx: SvcCtx, fail_cache: &ValChargeVolumeFailCache) -> Self {
        Self {
            parent_item_id: ctx.uad.items.id_by_key(fail_cache.parent_item_key),
            charge_volume: fail_cache.charge_volume,
            max_volume: fail_cache.max_volume,
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValChargeVolumeFailCache {
    pub(in crate::svc::vast) parent_item_key: ItemKey,
    pub(in crate::svc::vast) charge_item_key: ItemKey,
    pub(in crate::svc::vast) charge_volume: AttrVal,
    pub(in crate::svc::vast) max_volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_volume_fast(&mut self, kfs: &RSet<ItemKey>, ctx: SvcCtx) -> bool {
        for (module_item_key, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(ctx, *module_item_key, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail_cache) => {
                        let ret_fail = !kfs.contains(&fail_cache.charge_item_key);
                        cache.fail(fail_cache);
                        if ret_fail {
                            return false;
                        }
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail_cache) => {
                    if !kfs.contains(&fail_cache.charge_item_key) {
                        return false;
                    }
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeVolumeFail> {
        let mut charges = HashMap::new();
        for (module_item_key, cache) in self.mods_charge_volume.iter_mut() {
            match cache {
                ValCache::Todo(charge_volume) => match calculate_item_result(ctx, *module_item_key, *charge_volume) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail_cache) => {
                        if !kfs.contains(&fail_cache.charge_item_key) {
                            charges.insert(
                                ctx.uad.items.id_by_key(fail_cache.charge_item_key),
                                ValChargeVolumeChargeInfo::from_fail_cache(ctx, &fail_cache),
                            );
                        }
                        cache.fail(fail_cache);
                    }
                    _ => (),
                },
                ValCache::Pass(_) => (),
                ValCache::Fail(fail_cache) => {
                    if !kfs.contains(&fail_cache.charge_item_key) {
                        charges.insert(
                            ctx.uad.items.id_by_key(fail_cache.charge_item_key),
                            ValChargeVolumeChargeInfo::from_fail_cache(ctx, fail_cache),
                        );
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
    ctx: SvcCtx,
    module_item_key: ItemKey,
    charge_volume: AttrVal,
) -> ValCache<AttrVal, ValChargeVolumeFailCache> {
    let module = ctx.uad.items.get(module_item_key).get_module().unwrap();
    let module_capacity = match module.get_a_attrs() {
        Some(attrs) => match attrs.get(&ac::attrs::CAPACITY) {
            Some(module_capacity) => *module_capacity,
            None => OF(0.0),
        },
        None => return ValCache::Pass(charge_volume),
    };
    match charge_volume <= module_capacity {
        true => ValCache::Pass(charge_volume),
        false => ValCache::Fail(ValChargeVolumeFailCache {
            parent_item_key: module_item_key,
            charge_item_key: module.get_charge_item_key().unwrap(),
            charge_volume,
            max_volume: module_capacity,
        }),
    }
}
