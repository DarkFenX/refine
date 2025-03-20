use itertools::Itertools;
use smallvec::SmallVec;
use std::mem;

use crate::{
    defs::{AttrVal, EAttrId, OF, SolItemId},
    ec,
    sol::{
        SolSecZone, SolSecZoneCorruption,
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::SolUad,
    },
    util::{StMap, StSet},
};

const SEC_ZONE_COUNT: usize = mem::variant_count::<SolSecZone>();

pub struct SolValSecZoneFail {
    pub zone: SolSecZone,
    pub items: Vec<SolValSecZoneItemInfo>,
}

pub struct SolValSecZoneItemInfo {
    pub item_id: SolItemId,
    pub allowed_zones: Vec<SolSecZone>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_fitted_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_fitted)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_active)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> bool {
        class_check_fast(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        flags_check_fast(kfs, uad, calc, &self.sec_zone_unactivable)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_fitted_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Option<SolValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_fitted)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> Option<SolValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_online_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_active_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Option<SolValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_active)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unonlineable_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> Option<SolValSecZoneFail> {
        class_check_verbose(kfs, uad, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::sol::svc::vast) fn validate_sec_zone_unactivable_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Option<SolValSecZoneFail> {
        flags_check_verbose(kfs, uad, calc, &self.sec_zone_unactivable)
    }
}

// Disallowed/allowed flag validators
fn flags_check_fast(kfs: &StSet<SolItemId>, uad: &SolUad, calc: &mut SolCalc, items: &StSet<SolItemId>) -> bool {
    if items.is_empty() {
        return true;
    }
    match uad.sec_zone {
        SolSecZone::HiSec(corruption) => {
            for item_id in items.iter() {
                if is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_EMPIRE_SPACE)
                    || is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_HISEC)
                {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SolSecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                return false;
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted hisec
                        SolSecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ec::attrs::ALLOW_IN_FULLY_CORRUPTED_HISEC)
                                && !kfs.contains(item_id)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        SolSecZone::LowSec(corruption) => {
            for item_id in items.iter() {
                if is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_EMPIRE_SPACE) {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SolSecZoneCorruption::None => {
                            if !kfs.contains(item_id) {
                                return false;
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted lowsec
                        SolSecZoneCorruption::C5 => {
                            if !is_flag_set(uad, calc, item_id, &ec::attrs::ALLOW_IN_FULLY_CORRUPTED_LOWSEC)
                                && !kfs.contains(item_id)
                            {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        SolSecZone::Hazard => {
            for item_id in items.iter() {
                if is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_HAZARD) && !kfs.contains(item_id) {
                    return false;
                }
            }
        }
        // No limits for nullsec/w-space
        SolSecZone::NullSec | SolSecZone::WSpace => (),
    }
    true
}
fn flags_check_verbose(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    limitable_items: &StSet<SolItemId>,
) -> Option<SolValSecZoneFail> {
    if limitable_items.is_empty() {
        return None;
    }
    if matches!(uad.sec_zone, SolSecZone::NullSec | SolSecZone::WSpace) {
        return None;
    }
    let mut failed_items = Vec::new();
    for item_id in limitable_items.difference(kfs) {
        let allowed_zones = get_allowed_sec_zones(uad, calc, item_id);
        if !allowed_zones.iter().any(|v| compare_zones(&uad.sec_zone, v)) {
            failed_items.push(SolValSecZoneItemInfo {
                item_id: *item_id,
                allowed_zones: allowed_zones.to_vec(),
            });
        }
    }
    if failed_items.is_empty() {
        return None;
    }
    Some(SolValSecZoneFail {
        zone: uad.sec_zone,
        items: failed_items,
    })
}
fn is_flag_set(uad: &SolUad, calc: &mut SolCalc, item_id: &SolItemId, attr_id: &EAttrId) -> bool {
    match calc.get_item_attr_val_simple(uad, item_id, attr_id) {
        Some(val) => val != OF(0.0),
        None => match uad.items.get_item(item_id).unwrap().get_attrs().unwrap().get(attr_id) {
            Some(val) => *val != OF(0.0),
            None => false,
        },
    }
}
fn get_allowed_sec_zones(
    uad: &SolUad,
    calc: &mut SolCalc,
    item_id: &SolItemId,
) -> SmallVec<SolSecZone, SEC_ZONE_COUNT> {
    let mut allowed_zones = SmallVec::new();
    let disallow_empire = is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_EMPIRE_SPACE);
    // Hisec
    match disallow_empire || is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_HISEC) {
        true => {
            if is_flag_set(uad, calc, item_id, &ec::attrs::ALLOW_IN_FULLY_CORRUPTED_HISEC) {
                allowed_zones.push(SolSecZone::HiSec(SolSecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SolSecZone::HiSec(SolSecZoneCorruption::None)),
    }
    // Lowsec
    match disallow_empire {
        true => {
            if is_flag_set(uad, calc, item_id, &ec::attrs::ALLOW_IN_FULLY_CORRUPTED_LOWSEC) {
                allowed_zones.push(SolSecZone::LowSec(SolSecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SolSecZone::LowSec(SolSecZoneCorruption::None)),
    }
    // Null/w-space
    allowed_zones.extend([SolSecZone::NullSec, SolSecZone::WSpace]);
    // Zarzakh
    if !is_flag_set(uad, calc, item_id, &ec::attrs::DISALLOW_IN_HAZARD) {
        allowed_zones.push(SolSecZone::Hazard);
    }
    allowed_zones
}
fn compare_zones(actual: &SolSecZone, supported: &SolSecZone) -> bool {
    match actual {
        // No corruption in actual hisec zone - accept only uncorrupted hisec
        SolSecZone::HiSec(SolSecZoneCorruption::None) => {
            matches!(supported, SolSecZone::HiSec(SolSecZoneCorruption::None))
        }
        // For corrupted hisec zone accept any hisec, since items which work in uncorrupted hisec
        // work in corrupted hisec
        SolSecZone::HiSec(SolSecZoneCorruption::C5) => matches!(supported, SolSecZone::HiSec(_)),
        // Same logic as hisecs
        SolSecZone::LowSec(SolSecZoneCorruption::None) => {
            matches!(supported, SolSecZone::LowSec(SolSecZoneCorruption::None))
        }
        SolSecZone::LowSec(SolSecZoneCorruption::C5) => matches!(supported, SolSecZone::LowSec(_)),
        SolSecZone::NullSec => matches!(supported, SolSecZone::NullSec),
        SolSecZone::WSpace => matches!(supported, SolSecZone::WSpace),
        SolSecZone::Hazard => matches!(supported, SolSecZone::Hazard),
    }
}

// Security class validators
fn class_check_fast(kfs: &StSet<SolItemId>, uad: &SolUad, limitable_items: &StMap<SolItemId, AttrVal>) -> bool {
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
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    limitable_items: &StMap<SolItemId, AttrVal>,
) -> Option<SolValSecZoneFail> {
    if limitable_items.is_empty() {
        return None;
    }
    let current_class = zone_to_class(uad.sec_zone);
    let items = limitable_items
        .iter()
        .filter(|(item_id, item_sec_class)| **item_sec_class < current_class && !kfs.contains(item_id))
        .map(|(&item_id, &item_sec_class)| SolValSecZoneItemInfo {
            item_id,
            allowed_zones: class_to_allowed_zones(item_sec_class),
        })
        .collect_vec();
    if items.is_empty() {
        return None;
    }
    Some(SolValSecZoneFail {
        zone: uad.sec_zone,
        items,
    })
}
fn zone_to_class(zone: SolSecZone) -> AttrVal {
    match zone {
        SolSecZone::HiSec(_) => OF(2.0),
        SolSecZone::LowSec(_) => OF(1.0),
        _ => OF(0.0),
    }
}
fn class_to_allowed_zones(class: AttrVal) -> Vec<SolSecZone> {
    if class >= OF(2.0) {
        return vec![
            SolSecZone::HiSec(SolSecZoneCorruption::None),
            SolSecZone::LowSec(SolSecZoneCorruption::None),
            SolSecZone::NullSec,
            SolSecZone::WSpace,
            SolSecZone::Hazard,
        ];
    }
    if class >= OF(1.0) {
        return vec![
            SolSecZone::LowSec(SolSecZoneCorruption::None),
            SolSecZone::NullSec,
            SolSecZone::WSpace,
            SolSecZone::Hazard,
        ];
    }
    if class >= OF(0.0) {
        return vec![SolSecZone::NullSec, SolSecZone::WSpace, SolSecZone::Hazard];
    }
    Vec::new()
}
