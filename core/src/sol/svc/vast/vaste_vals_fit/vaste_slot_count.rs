use itertools::Itertools;

use crate::{
    defs::{Count, EAttrId, Idx, SolItemId},
    ec,
    sol::{
        svc::{
            calc::{AttrCalcError, SolCalc},
            vast::{SolStatSlot, SolVastFitData},
        },
        uad::{
            SolUad,
            fit::{SolFit, SolItemVec},
        },
    },
    util::{StMap, StSet, TriOption},
};

pub struct SolValSlotCountFail {
    pub used: Count,
    pub total: Option<Count>,
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
        validate_fast_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::MAX_SUBSYSTEMS, &fit.subsystems)
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
            &ec::attrs::MAX_ACTIVE_DRONES,
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
        validate_fast_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::FTR_TUBES, &self.fighters_online)
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
            &ec::attrs::FTR_SUPPORT_SLOTS,
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
            &ec::attrs::FTR_LIGHT_SLOTS,
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
            &ec::attrs::FTR_HEAVY_SLOTS,
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
            &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS,
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
            &ec::attrs::FTR_STANDUP_LIGHT_SLOTS,
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
            &ec::attrs::FTR_STANDUP_HEAVY_SLOTS,
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
            &ec::attrs::TURRET_SLOTS_LEFT,
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
            &ec::attrs::LAUNCHER_SLOTS_LEFT,
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
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::LOW_SLOTS, &fit.mods_low)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::UPGRADE_SLOTS_LEFT, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::MAX_SUBSYSTEMS, &fit.subsystems)
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
            &ec::attrs::MAX_ACTIVE_DRONES,
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
        validate_verbose_unordered_set(kfs, uad, calc, &fit.ship, &ec::attrs::FTR_TUBES, &self.fighters_online)
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
            &ec::attrs::FTR_SUPPORT_SLOTS,
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
            &ec::attrs::FTR_LIGHT_SLOTS,
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
            &ec::attrs::FTR_HEAVY_SLOTS,
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
            &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS,
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
            &ec::attrs::FTR_STANDUP_LIGHT_SLOTS,
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
            &ec::attrs::FTR_STANDUP_HEAVY_SLOTS,
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
            &ec::attrs::TURRET_SLOTS_LEFT,
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
            &ec::attrs::LAUNCHER_SLOTS_LEFT,
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
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::HI_SLOTS, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::MED_SLOTS, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        validate_verbose_ordered(kfs, uad, calc, &fit.ship, &ec::attrs::LOW_SLOTS, &fit.mods_low)
    }
}

fn validate_fast_unordered_set(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => output,
        TriOption::None => 0,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return true,
    };
    let used = users.len() as Count;
    used <= output
}
fn validate_fast_unordered_map<T>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &StMap<SolItemId, T>,
) -> bool {
    if users.is_subset(kfs) {
        return true;
    }
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => output,
        TriOption::None => 0,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return true,
    };
    let used = users.len() as Count;
    used <= output
}
fn validate_fast_ordered(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &SolItemVec,
) -> bool {
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => output,
        TriOption::None => 0,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return true,
    };
    let used = users.len() as Count;
    match kfs.is_empty() {
        true => used <= output,
        false => match used <= output {
            true => true,
            false => users.iter_ids_from(output as Idx).all(|v| kfs.contains(v)),
        },
    }
}

fn validate_verbose_unordered_set(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &StSet<SolItemId>,
) -> Option<SolValSlotCountFail> {
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => Some(output),
        TriOption::None => None,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return None,
    };
    let used = users.len() as Count;
    if used <= output.unwrap_or(0) {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail {
        used,
        total: output,
        users,
    })
}
fn validate_verbose_unordered_map<T>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &StMap<SolItemId, T>,
) -> Option<SolValSlotCountFail> {
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => Some(output),
        TriOption::None => None,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return None,
    };
    let used = users.len() as Count;
    if used <= output.unwrap_or(0) {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail {
        used,
        total: output,
        users,
    })
}
fn validate_verbose_ordered(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
    users: &SolItemVec,
) -> Option<SolValSlotCountFail> {
    let output = match get_output(uad, calc, output_item_id, output_attr_id) {
        TriOption::Some(output) => Some(output),
        TriOption::None => None,
        // Policy is to pass validations if some data is not available due to item being not loaded
        TriOption::Other => return None,
    };
    let used = users.len() as Count;
    let effective_output = output.unwrap_or(0);
    if used <= effective_output {
        return None;
    }
    let users = users
        .iter_ids_from(effective_output as Idx)
        .filter(|v| !kfs.contains(v))
        .copied()
        .collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail {
        used,
        total: output,
        users,
    })
}

fn get_output(
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
) -> TriOption<Count> {
    match output_item_id {
        Some(item_id) => match calc.get_item_attr_val_full(uad, item_id, output_attr_id) {
            Ok(val) => TriOption::Some(val.extra.into_inner().round() as Count),
            Err(error) => match error {
                AttrCalcError::ItemNotLoaded(_) => TriOption::Other,
                _ => TriOption::None,
            },
        },
        None => TriOption::None,
    }
}
