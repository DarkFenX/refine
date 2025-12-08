use std::collections::HashMap;

use super::shared::{is_attr_flag_set, is_oattr_flag_set};
use crate::{
    ad,
    def::{ItemId, OF},
    misc::{SecZone, SecZoneCorruption},
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::UItemKey,
    util::{RMap, RSet},
};

pub struct ValSecZoneFail {
    /// Solar system security zone.
    pub zone: SecZone,
    /// Map between IDs of items which cannot be used in current security zone, and a list of
    /// security zones they can be used in.
    pub items: HashMap<ItemId, Vec<SecZone>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_sec_zone_fitted_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(
            kfs,
            ctx,
            calc,
            &self.sec_zone_fitted,
            Some(&self.sec_zone_fitted_wspace_banned),
        )
    }
    pub(in crate::svc::vast) fn validate_sec_zone_online_fast(&self, kfs: &RSet<UItemKey>, ctx: SvcCtx) -> bool {
        class_check_fast(kfs, ctx, &self.sec_zone_online_class)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_active_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, ctx, calc, &self.sec_zone_active, None)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_unonlineable_fast(&self, kfs: &RSet<UItemKey>, ctx: SvcCtx) -> bool {
        class_check_fast(kfs, ctx, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_unactivable_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        flags_check_fast(kfs, ctx, calc, &self.sec_zone_unactivable, None)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_sec_zone_fitted_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(
            kfs,
            ctx,
            calc,
            &self.sec_zone_fitted,
            Some(&self.sec_zone_fitted_wspace_banned),
        )
    }
    pub(in crate::svc::vast) fn validate_sec_zone_online_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, ctx, &self.sec_zone_online_class)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_active_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, ctx, calc, &self.sec_zone_active, None)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_unonlineable_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValSecZoneFail> {
        class_check_verbose(kfs, ctx, &self.sec_zone_unonlineable_class)
    }
    pub(in crate::svc::vast) fn validate_sec_zone_unactivable_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValSecZoneFail> {
        flags_check_verbose(kfs, ctx, calc, &self.sec_zone_unactivable, None)
    }
}

// Disallowed/allowed flag validators
fn flags_check_fast(
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    items_main: &RSet<UItemKey>,
    items_wspace_banned: Option<&RSet<UItemKey>>,
) -> bool {
    match ctx.u_data.sec_zone {
        SecZone::HiSec(corruption) => {
            for &item_key in items_main.iter() {
                if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_empire_space)
                    || is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_hisec)
                {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(&item_key) {
                                return false;
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted hisec
                        SecZoneCorruption::C5 => {
                            if !is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_hisec)
                                && !kfs.contains(&item_key)
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
            if let Some(disallow_in_empire_space_key) = ctx.ac().disallow_in_empire_space {
                for &item_key in items_main.iter() {
                    if is_attr_flag_set(ctx, calc, item_key, disallow_in_empire_space_key) {
                        match corruption {
                            // No corruption in actual security zone - fail
                            SecZoneCorruption::None => {
                                if !kfs.contains(&item_key) {
                                    return false;
                                }
                            }
                            // If corrupted, check if module is allowed in corrupted lowsec
                            SecZoneCorruption::C5 => {
                                if !is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_lowsec)
                                    && !kfs.contains(&item_key)
                                {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
            true
        }
        SecZone::Hazard => {
            if let Some(disallow_in_hazard_key) = ctx.ac().disallow_in_hazard {
                for &item_key in items_main.iter() {
                    if is_attr_flag_set(ctx, calc, item_key, disallow_in_hazard_key) && !kfs.contains(&item_key) {
                        return false;
                    }
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
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    items_main: &RSet<UItemKey>,
    items_wspace_banned: Option<&RSet<UItemKey>>,
) -> Option<ValSecZoneFail> {
    let mut failed_item_keys = Vec::new();
    match ctx.u_data.sec_zone {
        SecZone::HiSec(corruption) => {
            for &item_key in items_main.iter() {
                if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_empire_space)
                    || is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_hisec)
                {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(&item_key) {
                                failed_item_keys.push(item_key);
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted hisec
                        SecZoneCorruption::C5 => {
                            if !is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_hisec)
                                && !kfs.contains(&item_key)
                            {
                                failed_item_keys.push(item_key);
                            }
                        }
                    }
                }
            }
        }
        SecZone::LowSec(corruption) => {
            for &item_key in items_main.iter() {
                if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_empire_space) {
                    match corruption {
                        // No corruption in actual security zone - fail
                        SecZoneCorruption::None => {
                            if !kfs.contains(&item_key) {
                                failed_item_keys.push(item_key);
                            }
                        }
                        // If corrupted, check if module is allowed in corrupted lowsec
                        SecZoneCorruption::C5 => {
                            if !is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_lowsec)
                                && !kfs.contains(&item_key)
                            {
                                failed_item_keys.push(item_key);
                            }
                        }
                    }
                }
            }
        }
        SecZone::Hazard => {
            for &item_key in items_main.iter() {
                if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_hazard) && !kfs.contains(&item_key) {
                    failed_item_keys.push(item_key);
                }
            }
        }
        // No limits for nullsec
        SecZone::NullSec => (),
        // Supercap ban for w-space
        SecZone::WSpace => {
            if let Some(items_wspace_banned) = items_wspace_banned {
                failed_item_keys.extend(items_wspace_banned.difference(kfs).copied());
            }
        }
    };
    match failed_item_keys.is_empty() {
        true => None,
        false => Some(ValSecZoneFail {
            zone: ctx.u_data.sec_zone,
            items: failed_item_keys
                .iter()
                .map(|&item_key| {
                    (
                        ctx.u_data.items.id_by_key(item_key),
                        get_allowed_sec_zones(ctx, calc, item_key, items_wspace_banned),
                    )
                })
                .collect(),
        }),
    }
}
fn get_allowed_sec_zones(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    items_wspace_banned: Option<&RSet<UItemKey>>,
) -> Vec<SecZone> {
    let mut allowed_zones = Vec::new();
    let disallow_empire = is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_empire_space);
    // Hisec
    match disallow_empire || is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_hisec) {
        true => {
            if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_hisec) {
                allowed_zones.push(SecZone::HiSec(SecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SecZone::HiSec(SecZoneCorruption::None)),
    }
    // Lowsec
    match disallow_empire {
        true => {
            if is_oattr_flag_set(ctx, calc, item_key, ctx.ac().allow_in_fully_corrupted_lowsec) {
                allowed_zones.push(SecZone::LowSec(SecZoneCorruption::C5))
            }
        }
        false => allowed_zones.push(SecZone::LowSec(SecZoneCorruption::None)),
    }
    // Nullsec
    allowed_zones.push(SecZone::NullSec);
    // W-space
    if match items_wspace_banned {
        Some(items_wspace_banned) => !items_wspace_banned.contains(&item_key),
        None => true,
    } {
        allowed_zones.push(SecZone::WSpace);
    }
    // Zarzakh
    if !is_oattr_flag_set(ctx, calc, item_key, ctx.ac().disallow_in_hazard) {
        allowed_zones.push(SecZone::Hazard);
    }
    allowed_zones
}

// Security class validators
fn class_check_fast(kfs: &RSet<UItemKey>, ctx: SvcCtx, limitable_items: &RMap<UItemKey, ad::AAttrVal>) -> bool {
    if limitable_items.is_empty() {
        return true;
    }
    let current_sec_class = zone_to_class(ctx.u_data.sec_zone);
    for (item_key, &item_sec_class) in limitable_items.iter() {
        if current_sec_class > item_sec_class && !kfs.contains(item_key) {
            return false;
        }
    }
    true
}
fn class_check_verbose(
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    limitable_items: &RMap<UItemKey, ad::AAttrVal>,
) -> Option<ValSecZoneFail> {
    if limitable_items.is_empty() {
        return None;
    }
    let current_class = zone_to_class(ctx.u_data.sec_zone);
    let items: HashMap<_, _> = limitable_items
        .iter()
        .filter(|(item_key, item_sec_class)| **item_sec_class < current_class && !kfs.contains(item_key))
        .map(|(&item_key, &item_sec_class)| {
            (
                ctx.u_data.items.id_by_key(item_key),
                class_to_allowed_zones(item_sec_class),
            )
        })
        .collect();
    match items.is_empty() {
        true => None,
        false => Some(ValSecZoneFail {
            zone: ctx.u_data.sec_zone,
            items,
        }),
    }
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
