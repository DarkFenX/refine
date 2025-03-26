use itertools::Itertools;
use ordered_float::OrderedFloat as OF;
use smallvec::SmallVec;

use crate::{
    ac, ad,
    sol::{
        ItemId, SecZone, SecZoneCorruption,
        svc::{calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::{StMap, StSet},
};

use super::shared::is_flag_set;

const SEC_ZONE_COUNT: usize = std::mem::variant_count::<SecZone>();

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
        kfs: &StSet<ItemId>,
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
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_fast(&self, kfs: &StSet<ItemId>, uad: &Uad) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_active, None)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
    ) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_unactivable, None)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_fitted_verbose(
        &self,
        kfs: &StSet<ItemId>,
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
        kfs: &StSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_active, None)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_unactivable, None)
    }
}

// Disallowed/allowed flag validators
fn flags_check_fast(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    items_main: &StSet<ItemId>,
    items_wspace_banned: Option<&StSet<ItemId>>,
) -> bool {
    if items_main.is_empty()
        && match items_wspace_banned {
            Some(items_wspace_banned) => !items_wspace_banned.is_empty(),
            None => true,
        }
    {
        return true;
    }
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
        SecZone::WSpace => match items_wspace_banned {
            Some(items_wspace_banned) => items_wspace_banned.is_subset(kfs),
            None => true,
        },
    }
}
fn flags_check_verbose(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    items_main: &StSet<ItemId>,
    items_wspace_banned: Option<&StSet<ItemId>>,
) -> Option<ValSecZoneFail> {
    if items_main.is_empty() {
        return None;
    }
    if matches!(uad.sec_zone, SecZone::NullSec | SecZone::WSpace) {
        return None;
    }
    let mut failed_items = Vec::new();
    for item_id in items_main.difference(kfs) {
        let allowed_zones = get_allowed_sec_zones(uad, calc, item_id, items_wspace_banned);
        if !allowed_zones.iter().any(|v| compare_zones(&uad.sec_zone, v)) {
            failed_items.push(ValSecZoneItemInfo {
                item_id: *item_id,
                allowed_zones: allowed_zones.to_vec(),
            });
        }
    }
    if failed_items.is_empty() {
        return None;
    }
    Some(ValSecZoneFail {
        zone: uad.sec_zone,
        items: failed_items,
    })
}
fn get_allowed_sec_zones(
    uad: &Uad,
    calc: &mut Calc,
    item_id: &ItemId,
    items_wspace_banned: Option<&StSet<ItemId>>,
) -> SmallVec<SecZone, SEC_ZONE_COUNT> {
    let mut allowed_zones = SmallVec::new();
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
fn compare_zones(actual: &SecZone, supported: &SecZone) -> bool {
    match actual {
        // No corruption in actual hisec zone - accept only uncorrupted hisec
        SecZone::HiSec(SecZoneCorruption::None) => {
            matches!(supported, SecZone::HiSec(SecZoneCorruption::None))
        }
        // For corrupted hisec zone accept any hisec, since items which work in uncorrupted hisec
        // work in corrupted hisec
        SecZone::HiSec(SecZoneCorruption::C5) => matches!(supported, SecZone::HiSec(_)),
        // Same logic as hisecs
        SecZone::LowSec(SecZoneCorruption::None) => {
            matches!(supported, SecZone::LowSec(SecZoneCorruption::None))
        }
        SecZone::LowSec(SecZoneCorruption::C5) => matches!(supported, SecZone::LowSec(_)),
        SecZone::NullSec => matches!(supported, SecZone::NullSec),
        SecZone::WSpace => matches!(supported, SecZone::WSpace),
        SecZone::Hazard => matches!(supported, SecZone::Hazard),
    }
}

// Security class validators
fn class_check_fast(kfs: &StSet<ItemId>, uad: &Uad, limitable_items: &StMap<ItemId, ad::AAttrVal>) -> bool {
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
    kfs: &StSet<ItemId>,
    uad: &Uad,
    limitable_items: &StMap<ItemId, ad::AAttrVal>,
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
