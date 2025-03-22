use itertools::Itertools;

use crate::{
    consts,
    defs::{AttrVal, OF, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
    util::StSet,
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
        let max = get_max_resource(uad, calc, &fit.ship, &consts::attrs::DRONE_BANDWIDTH).unwrap_or(OF(0.0));
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
        let max = get_max_resource(uad, calc, &fit.ship, &consts::attrs::DRONE_BANDWIDTH);
        let effective_max = max.unwrap_or(OF(0.0));
        let users = self
            .drones_bandwidth
            .iter()
            .filter(|(item_id, item_use)| **item_use > effective_max && !kfs.contains(item_id))
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
