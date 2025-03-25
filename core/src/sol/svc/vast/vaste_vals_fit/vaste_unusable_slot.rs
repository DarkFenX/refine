use itertools::Itertools;

use crate::{
    ac, ad,
    sol::{
        Count, ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::Fit},
    },
    util::StSet,
};

use super::shared::get_max_slots;

pub struct ValUnusableSlotFail {
    pub max: Option<Count>,
    pub users: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_slot_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            &fit.drones,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(kfs, uad, calc, &fit.ship, &ac::attrs::FTR_TUBES, &fit.fighters)
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_support_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_light_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_heavy_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_support_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_light_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_heavy_fighter_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_drone_slot_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.character,
            &ac::attrs::MAX_ACTIVE_DRONES,
            &fit.drones,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(kfs, uad, calc, &fit.ship, &ac::attrs::FTR_TUBES, &fit.fighters)
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_support_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_light_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_heavy_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_support_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_light_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_unlaunchable_standup_heavy_fighter_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &fit.ship,
            &ac::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters,
        )
    }
}

fn validate_fast(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_id: &Option<ItemId>,
    max_a_attr_id: &ad::AAttrId,
    users: &StSet<ItemId>,
) -> bool {
    if users.is_empty() {
        return true;
    }
    let max = get_max_slots(uad, calc, max_item_id, max_a_attr_id).unwrap_or(0);
    if max > 0 {
        return true;
    }
    users.is_subset(kfs)
}
fn validate_verbose(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    max_item_id: &Option<ItemId>,
    max_a_attr_id: &ad::AAttrId,
    users: &StSet<ItemId>,
) -> Option<ValUnusableSlotFail> {
    if users.is_empty() {
        return None;
    }
    let max = get_max_slots(uad, calc, max_item_id, max_a_attr_id);
    if max.unwrap_or(0) > 0 {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(ValUnusableSlotFail { max, users })
}
