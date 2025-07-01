use std::collections::HashMap;

use crate::{
    def::{ItemGrpId, ItemId, ItemKey},
    svc::{
        misc::SvcCtx,
        vast::{ValCache, VastFitData},
    },
    util::RSet,
};

pub struct ValChargeGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeGroupChargeInfo>,
}

pub struct ValChargeGroupChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Group ID of current charge.
    pub charge_group_id: ItemGrpId,
    /// Group IDs allowed by containing module.
    pub allowed_group_ids: Vec<ItemGrpId>,
}
impl ValChargeGroupChargeInfo {
    fn from_fail_cache(ctx: SvcCtx, fail_cache: &ValChargeGroupFailCache) -> Self {
        Self {
            parent_item_id: ctx.uad.items.id_by_key(fail_cache.parent_item_key),
            charge_group_id: fail_cache.charge_group_id,
            allowed_group_ids: fail_cache.allowed_group_ids.clone(),
        }
    }
}

#[derive(Clone)]
pub(in crate::svc::vast) struct ValChargeGroupFailCache {
    pub(in crate::svc::vast) parent_item_key: ItemKey,
    pub(in crate::svc::vast) charge_item_key: ItemKey,
    pub(in crate::svc::vast) charge_group_id: ItemGrpId,
    pub(in crate::svc::vast) allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_group_fast(&mut self, kfs: &RSet<ItemKey>, ctx: SvcCtx) -> bool {
        for (module_item_key, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(ctx, *module_item_key) {
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
    pub(in crate::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeGroupFail> {
        let mut charges = HashMap::new();
        for (module_item_key, cache) in self.mods_charge_group.iter_mut() {
            match cache {
                ValCache::Todo(_) => match calculate_item_result(ctx, *module_item_key) {
                    ValCache::Pass(pass) => cache.pass(pass),
                    ValCache::Fail(fail_cache) => {
                        if !kfs.contains(&fail_cache.charge_item_key) {
                            charges.insert(
                                ctx.uad.items.id_by_key(fail_cache.charge_item_key),
                                ValChargeGroupChargeInfo::from_fail_cache(ctx, &fail_cache),
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
                            ValChargeGroupChargeInfo::from_fail_cache(ctx, fail_cache),
                        );
                    }
                }
            }
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeGroupFail { charges }),
        }
    }
}

fn calculate_item_result(ctx: SvcCtx, module_item_key: ItemKey) -> ValCache<(), ValChargeGroupFailCache> {
    let module = ctx.uad.items.get(module_item_key).get_module().unwrap();
    let charge_item_key = match module.get_charge_item_key() {
        Some(charge_item_key) => charge_item_key,
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
    let charge_group_id = match ctx.uad.items.get(charge_item_key).get_a_group_id() {
        Some(charge_group_id) => charge_group_id,
        None => return ValCache::Pass(()),
    };
    match allowed_group_ids.contains(&charge_group_id) {
        true => ValCache::Pass(()),
        false => ValCache::Fail(ValChargeGroupFailCache {
            parent_item_key: module_item_key,
            charge_item_key,
            charge_group_id,
            allowed_group_ids,
        }),
    }
}
