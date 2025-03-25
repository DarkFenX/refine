use itertools::Itertools;
use ordered_float::OrderedFloat as OF;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::Fit},
    },
    util::StSet,
};

use super::shared::get_max_resource;

pub struct ValUnusableResFail {
    pub max: Option<AttrVal>,
    pub users: Vec<ValUnusableResItemInfo>,
}

pub struct ValUnusableResItemInfo {
    pub item_id: ItemId,
    pub used: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_bandwidth_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        if self.drones_bandwidth.is_empty() {
            return true;
        }
        let max = get_max_resource(uad, calc, &fit.ship, &ac::attrs::DRONE_BANDWIDTH).unwrap_or(OF(0.0));
        for (item_id, &item_use) in self.drones_bandwidth.iter() {
            if item_use > max && !kfs.contains(item_id) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_bandwidth_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableResFail> {
        if self.drones_bandwidth.is_empty() {
            return None;
        }
        let max = get_max_resource(uad, calc, &fit.ship, &ac::attrs::DRONE_BANDWIDTH);
        let effective_max = max.unwrap_or(OF(0.0));
        let users = self
            .drones_bandwidth
            .iter()
            .filter(|(item_id, item_use)| **item_use > effective_max && !kfs.contains(item_id))
            .map(|(&item_id, &item_use)| ValUnusableResItemInfo {
                item_id,
                used: item_use,
            })
            .collect_vec();
        if users.is_empty() {
            return None;
        }
        Some(ValUnusableResFail { max, users })
    }
}
