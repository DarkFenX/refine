use itertools::Itertools;

use crate::{
    defs::{Count, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
    util::{StSet, TriOption},
};

use super::shared::get_max_slots;

pub struct SolValUnusableSlotFail {
    pub max: Option<Count>,
    pub users: Vec<SolItemId>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_slot_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        if fit.drones.is_empty() {
            return true;
        }
        match get_max_slots(uad, calc, &fit.character, &ec::attrs::MAX_ACTIVE_DRONES) {
            TriOption::Some(max) if max > 0 => return true,
            // Policy is to pass validations if some data is not available due to item being not loaded
            TriOption::Other => return true,
            _ => (),
        };
        fit.drones.is_subset(kfs)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_slot_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        if fit.drones.is_empty() {
            return None;
        }
        let max = match get_max_slots(uad, calc, &fit.character, &ec::attrs::MAX_ACTIVE_DRONES) {
            TriOption::Some(max) => match max > 0 {
                true => return None,
                false => Some(max),
            },
            TriOption::None => None,
            // Policy is to pass validations if some data is not available due to item being not loaded
            TriOption::Other => return None,
        };
        let users = fit.drones.difference(kfs).copied().collect_vec();
        if users.is_empty() {
            return None;
        }
        Some(SolValUnusableSlotFail { max, users })
    }
}
