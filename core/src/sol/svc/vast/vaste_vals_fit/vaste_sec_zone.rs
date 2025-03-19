use itertools::Itertools;

use crate::{
    AttrVal,
    defs::{OF, SolItemId},
    sol::{SolSecZone, SolSecZoneCorruption, svc::vast::SolVastFitData, uad::SolUad},
    util::StSet,
};

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
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> bool {
        if self.mods_svcs_sec_class_online.is_empty() {
            return true;
        }
        let current_sec_class = zone_to_class(uad.sec_zone);
        for (item_id, &item_sec_class) in self.mods_svcs_sec_class_online.iter() {
            if current_sec_class > item_sec_class && !kfs.contains(item_id) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_sec_zone_online_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
    ) -> Option<SolValSecZoneFail> {
        if self.mods_svcs_sec_class_online.is_empty() {
            return None;
        }
        let current_class = zone_to_class(uad.sec_zone);
        let items = self
            .mods_svcs_sec_class_online
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
            SolSecZone::HiSec(SolSecZoneCorruption::Any),
            SolSecZone::LowSec(SolSecZoneCorruption::Any),
            SolSecZone::NullSec,
            SolSecZone::WSpace,
            SolSecZone::Hazard,
        ];
    }
    if class >= OF(1.0) {
        return vec![
            SolSecZone::LowSec(SolSecZoneCorruption::Any),
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
