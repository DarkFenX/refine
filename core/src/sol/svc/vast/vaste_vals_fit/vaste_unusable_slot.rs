use itertools::Itertools;

use crate::{
    EAttrId,
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
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.character,
            &ec::attrs::MAX_ACTIVE_DRONES,
            &fit.drones,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(kfs, uad, calc, &fit.ship, &ec::attrs::FTR_TUBES, &fit.fighters)
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_support_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_light_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_heavy_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_support_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_light_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_heavy_fighter_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_slot_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.character,
            &ec::attrs::MAX_ACTIVE_DRONES,
            &fit.drones,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(kfs, uad, calc, &fit.ship, &ec::attrs::FTR_TUBES, &fit.fighters)
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_support_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_light_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_heavy_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_support_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_light_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_heavy_fighter_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ec::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters,
        )
    }
}

fn validate_fast(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> bool {
    if users.is_empty() {
        return true;
    }
    match get_max_slots(uad, calc, max_item_id, max_attr_id) {
        TriOption::Some(max) if max > 0 => return true,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return true,
        _ => (),
    };
    users.is_subset(kfs)
}
fn validate_verbose(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> Option<SolValUnusableSlotFail> {
    if users.is_empty() {
        return None;
    }
    let max = match get_max_slots(uad, calc, max_item_id, max_attr_id) {
        TriOption::Some(max) => match max > 0 {
            true => return None,
            false => Some(max),
        },
        TriOption::None => None,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return None,
    };
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValUnusableSlotFail { max, users })
}
