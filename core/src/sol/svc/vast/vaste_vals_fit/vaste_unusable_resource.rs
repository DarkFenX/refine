use itertools::Itertools;

use crate::{
    defs::{AttrVal, OF, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
    util::{StSet, TriOption},
};

use super::shared::get_max_resource;

pub struct SolValUnusableResFail {
    pub max: Option<AttrVal>,
    pub users: Vec<SolValUnusableResItemInfo>,
}

pub struct SolValUnusableResItemInfo {
    pub item_id: SolItemId,
    pub used: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_bandwidth_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        if self.drones_bandwidth.is_empty() {
            return true;
        }
        let max = match get_max_resource(uad, calc, &fit.ship, &ec::attrs::DRONE_BANDWIDTH) {
            TriOption::Some(value) => value,
            TriOption::None => OF(0.0),
            // Policy is to pass validations if some data is not available due to item being not loaded
            TriOption::Other => return true,
        };
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
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableResFail> {
        if self.drones_bandwidth.is_empty() {
            return None;
        }
        let max = match get_max_resource(uad, calc, &fit.ship, &ec::attrs::DRONE_BANDWIDTH) {
            TriOption::Some(value) => Some(value),
            TriOption::None => None,
            // Policy is to pass validations if some data is not available due to item being not loaded
            TriOption::Other => return None,
        };
        let effective_max = max.unwrap_or(OF(0.0));
        let users = self
            .drones_bandwidth
            .iter()
            .filter(|(item_id, &item_use)| item_use > effective_max && !kfs.contains(item_id))
            .map(|(&item_id, &item_use)| SolValUnusableResItemInfo {
                item_id,
                used: item_use,
            })
            .collect_vec();
        if users.is_empty() {
            return None;
        }
        Some(SolValUnusableResFail { max, users })
    }
}
