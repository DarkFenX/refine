use std::collections::HashMap;

use super::shared::get_max_resource;
use crate::{
    def::{AttrVal, ItemId, OF},
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{UFit, UItemId},
    util::RSet,
};

pub struct ValUnusableResFail {
    /// Max available resource (e.g. amount of CPU produced by ship).
    pub max: Option<AttrVal>,
    /// Map with consumer item IDs and amount they consume.
    pub users: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_bandwidth_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        if self.drones_bandwidth.is_empty() {
            return true;
        }
        let max = get_max_resource(ctx, calc, fit.ship, ctx.ac().drone_bandwidth).unwrap_or(OF(0.0));
        for (item_key, &item_use) in self.drones_bandwidth.iter() {
            if item_use > max && !kfs.contains(item_key) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_bandwidth_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableResFail> {
        if self.drones_bandwidth.is_empty() {
            return None;
        }
        let max = get_max_resource(ctx, calc, fit.ship, ctx.ac().drone_bandwidth);
        let effective_max = max.unwrap_or(OF(0.0));
        let users: HashMap<_, _> = self
            .drones_bandwidth
            .iter()
            .filter(|(item_key, item_use)| **item_use > effective_max && !kfs.contains(item_key))
            .map(|(item_key, item_use)| (ctx.u_data.items.ext_id_by_int_id(*item_key), *item_use))
            .collect();
        match users.is_empty() {
            true => None,
            false => Some(ValUnusableResFail { max, users }),
        }
    }
}
