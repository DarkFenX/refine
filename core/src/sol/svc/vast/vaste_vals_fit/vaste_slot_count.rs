use itertools::Itertools;

use crate::{
    consts,
    defs::{Count, EAttrId, Idx, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{
            SolUad,
            fit::{SolFit, SolItemVec},
        },
    },
    util::{StMap, StSet},
};

use super::shared::get_max_slots;

pub struct SolValSlotCountFail {
    pub used: Count,
    pub max: Option<Count>,
    pub users: Vec<SolItemId>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, &fit.ship, &consts::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, &fit.ship, &consts::attrs::SERVICE_SLOTS, &fit.services)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::MAX_SUBSYSTEMS,
            &fit.subsystems,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_map(
            kfs,
            uad,
            calc,
            &fit.character,
            &consts::attrs::MAX_ACTIVE_DRONES,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_TUBES,
            &self.fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::TURRET_SLOTS_LEFT,
            &self.mods_turret,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::LAUNCHER_SLOTS_LEFT,
            &self.mods_launcher,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::LOW_SLOTS, &fit.mods_low)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, &fit.ship, &consts::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_service_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, &fit.ship, &consts::attrs::SERVICE_SLOTS, &fit.services)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::MAX_SUBSYSTEMS,
            &fit.subsystems,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_map(
            kfs,
            uad,
            calc,
            &fit.character,
            &consts::attrs::MAX_ACTIVE_DRONES,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_TUBES,
            &self.fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_SUPPORT_SLOTS,
            &self.support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_LIGHT_SLOTS,
            &self.light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_HEAVY_SLOTS,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_SUPPORT_SLOTS,
            &self.standup_support_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_LIGHT_SLOTS,
            &self.standup_light_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::FTR_STANDUP_HEAVY_SLOTS,
            &self.standup_heavy_fighters_online,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::TURRET_SLOTS_LEFT,
            &self.mods_turret,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            uad,
            calc,
            &fit.ship,
            &consts::attrs::LAUNCHER_SLOTS_LEFT,
            &self.mods_launcher,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &consts::attrs::LOW_SLOTS, &fit.mods_low)
    }
}

fn validate_fast_unordered_set(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id).unwrap_or(0);
    used <= max
}
fn validate_fast_unordered_map<T>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StMap<SolItemId, T>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id).unwrap_or(0);
    used <= max
}
fn validate_fast_ordered(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &SolItemVec,
) -> bool {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id).unwrap_or(0);
    match kfs.is_empty() {
        true => used <= max,
        false => match used <= max {
            true => true,
            false => users.iter_ids_from(max as Idx).all(|v| kfs.contains(v)),
        },
    }
}

fn validate_verbose_unordered_set(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> Option<SolValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail { used, max, users })
}
fn validate_verbose_unordered_map<T>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &StMap<SolItemId, T>,
) -> Option<SolValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail { used, max, users })
}
fn validate_verbose_ordered(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
    users: &SolItemVec,
) -> Option<SolValSlotCountFail> {
    let used = users.len() as Count;
    let max = get_max_slots(uad, calc, max_item_id, max_attr_id);
    let effective_max = max.unwrap_or(0);
    if used <= effective_max {
        return None;
    }
    let users = users
        .iter_ids_from(effective_max as Idx)
        .filter(|v| !kfs.contains(v))
        .copied()
        .collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail { used, max, users })
}
