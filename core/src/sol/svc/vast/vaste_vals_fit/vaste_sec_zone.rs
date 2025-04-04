use itertools::Itertools;
use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        ItemId, SecZone, SecZoneCorruption,
        svc::{calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::{RMap, RSet},
};

use super::shared::is_flag_set;

pub struct ValSecZoneFail {
    pub zone: SecZone,
    pub items: Vec<ValSecZoneItemInfo>,
}

pub struct ValSecZoneItemInfo {
    pub item_id: ItemId,
    pub allowed_zones: Vec<SecZone>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_fitted_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(
            kfs,
            uad,
            calc,
            &self.sec_zone_fitted,
            Some(&self.sec_zone_fitted_wspace_banned),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_fast(&self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_active, None)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_fast(&self, kfs: &RSet<ItemId>, uad: &Uad) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_unactivable, None)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_fitted_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(
            kfs,
            uad,
            calc,
            &self.sec_zone_fitted,
            Some(&self.sec_zone_fitted_wspace_banned),
        )
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_active, None)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_unactivable, None)
    }
}

// Disallowed/allowed flag validators
fn flags_check_fast(
    kfs: &RSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    items_main: &RSet<ItemId>,
    items_wspace_banned: Option<&RSet<ItemId>>,
) -> bool {
    match uad.sec_zone {
        SecZone::HiSec(corruption) => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_EMPIRE_SPACE)
                    || is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HISEC)
                {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                return false;
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted hisec
                        SecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_HISEC)
                                && !kfs.contains(item_id)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
            true
        }
        SecZone::LowSec(corruption) => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_EMPIRE_SPACE) {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                return false;
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted lowsec
                        SecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_LOWSEC)
                                && !kfs.contains(item_id)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
            true
        }
        SecZone::Hazard => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HAZARD) && !kfs.contains(item_id) {
                    return false;
                }
            }
            true
        }
        // No limits for nullsec
        SecZone::NullSec => true,
        // Supercap ban for w-space
        SecZone::WSpace => match items_wspace_banned {
            Some(items_wspace_banned) => items_wspace_banned.is_subset(kfs),
            None => true,
        },
    }
}
fn flags_check_verbose(
    kfs: &RSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    items_main: &RSet<ItemId>,
    items_wspace_banned: Option<&RSet<ItemId>>,
) -> Option<ValSecZoneFail> {
    let mut fails = Vec::new();
    match uad.sec_zone {
        SecZone::HiSec(corruption) => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_EMPIRE_SPACE)
                    || is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HISEC)
                {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                fails.push(*item_id);
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted hisec
                        SecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_HISEC)
                                && !kfs.contains(item_id)
                            {
                                fails.push(*item_id);
                            }
                        }
                    }
                }
            }
        }
        SecZone::LowSec(corruption) => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_EMPIRE_SPACE) {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                fails.push(*item_id);
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted lowsec
                        SecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_LOWSEC)
                                && !kfs.contains(item_id)
                            {
                                fails.push(*item_id);
                            }
                        }
                    }
                }
            }
        }
        SecZone::Hazard => {
            for item_id in items_main.iter() {
                if is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HAZARD) && !kfs.contains(item_id) {
                    fails.push(*item_id);
                }
            }
        }
        // No limits for nullsec
        SecZone::NullSec => (),
        // Supercap ban for w-space
        SecZone::WSpace => {
            if let Some(items_wspace_banned) = items_wspace_banned {
                fails.extend(items_wspace_banned.difference(kfs).copied());
            }
        }
    };
    if fails.is_empty() {
        return None;
    }
    Some(ValSecZoneFail {
        zone: uad.sec_zone,
        items: fails
            .iter()
            .map(|v| ValSecZoneItemInfo {
                item_id: *v,
                allowed_zones: get_allowed_sec_zones(uad, calc, v, items_wspace_banned),
            })
            .collect(),
    })
}
fn get_allowed_sec_zones(
    uad: &Uad,
    calc: &mut Calc,
    item_id: &ItemId,
    items_wspace_banned: Option<&RSet<ItemId>>,
) -> Vec<SecZone> {
    let mut allowed_zones = Vec::new();
    let disallow_empire = is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_EMPIRE_SPACE);
    // Hisec
    match disallow_empire || is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HISEC) {
        true => {
            if is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_HISEC) {
                allowed_zones.push(SecZone::HiSec(SecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SecZone::HiSec(SecZoneCorruption::None)),
    }
    // Lowsec
    match disallow_empire {
        true => {
            if is_flag_set(uad, calc, item_id, &ac::attrs::ALLOW_IN_FULLY_CORRUPTED_LOWSEC) {
                allowed_zones.push(SecZone::LowSec(SecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SecZone::LowSec(SecZoneCorruption::None)),
    }
    // Nullsec
    allowed_zones.push(SecZone::NullSec);
    // W-space
    if match items_wspace_banned {
        Some(items_wspace_banned) => !items_wspace_banned.contains(item_id),
        None => true,
    } {
        allowed_zones.push(SecZone::WSpace);
    }
    // Zarzakh
    if !is_flag_set(uad, calc, item_id, &ac::attrs::DISALLOW_IN_HAZARD) {
        allowed_zones.push(SecZone::Hazard);
    }
    allowed_zones
}

// Security class validators
fn class_check_fast(kfs: &RSet<ItemId>, uad: &Uad, limitable_items: &RMap<ItemId, ad::AAttrVal>) -> bool {
    if limitable_items.is_empty() {
        return true;
    }
    let current_sec_class = zone_to_class(uad.sec_zone);
    for (item_id, &item_sec_class) in limitable_items.iter() {
        if current_sec_class > item_sec_class && !kfs.contains(item_id) {
            return false;
        }
    }
    true
}
fn class_check_verbose(
    kfs: &RSet<ItemId>,
    uad: &Uad,
    limitable_items: &RMap<ItemId, ad::AAttrVal>,
) -> Option<ValSecZoneFail> {
    if limitable_items.is_empty() {
        return None;
    }
    let current_class = zone_to_class(uad.sec_zone);
    let items = limitable_items
        .iter()
        .filter(|(item_id, item_sec_class)| **item_sec_class < current_class && !kfs.contains(item_id))
        .map(|(&item_id, &item_sec_class)| ValSecZoneItemInfo {
            item_id,
            allowed_zones: class_to_allowed_zones(item_sec_class),
        })
        .collect_vec();
    if items.is_empty() {
        return None;
    }
    Some(ValSecZoneFail {
        zone: uad.sec_zone,
        items,
    })
}
fn zone_to_class(zone: SecZone) -> ad::AAttrVal {
    match zone {
        SecZone::HiSec(_) => OF(2.0),
        SecZone::LowSec(_) => OF(1.0),
        _ => OF(0.0),
    }
}
fn class_to_allowed_zones(class: ad::AAttrVal) -> Vec<SecZone> {
    if class >= OF(2.0) {
        return vec![
            SecZone::HiSec(SecZoneCorruption::None),
            SecZone::LowSec(SecZoneCorruption::None),
            SecZone::NullSec,
            SecZone::WSpace,
            SecZone::Hazard,
        ];
    }
    if class >= OF(1.0) {
        return vec![
            SecZone::LowSec(SecZoneCorruption::None),
            SecZone::NullSec,
            SecZone::WSpace,
            SecZone::Hazard,
        ];
    }
    if class >= OF(0.0) {
        return vec![SecZone::NullSec, SecZone::WSpace, SecZone::Hazard];
    }
    Vec::new()
}
