use crate::{
    defs::{AttrVal, EAttrId, OF, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
    util::{StSet, round},
};

use super::shared::get_max_resource;

pub struct SolValResFail {
    pub used: AttrVal,
    pub max: Option<AttrVal>,
    pub users: Vec<SolValResItemInfo>,
}

pub struct SolValResItemInfo {
    pub item_id: SolItemId,
    pub used: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.rigs_rigslot_calibration.iter(),
            &ec::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_volume.iter(),
            &ec::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ec::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ec::attrs::FTR_CAPACITY,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.rigs_rigslot_calibration.iter(),
            &ec::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_volume.iter(),
            &ec::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ec::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ec::attrs::FTR_CAPACITY,
        )
    }
}

fn validate_fast_fitting<'a>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    item_ids: impl Iterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
    max_attr_id: &EAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val_simple(uad, item_id, use_attr_id) {
            Some(item_use) => item_use,
            None => continue,
        };
        if force_pass && item_use > OF(0.0) && !kfs.contains(item_id) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(uad, calc, &fit.ship, max_attr_id).unwrap_or(OF(0.0));
    round(total_use, 2) <= max
}
fn validate_fast_other<'a>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    items: impl Iterator<Item = (&'a SolItemId, &'a AttrVal)>,
    max_attr_id: &EAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for (item_id, &item_use) in items {
        if force_pass && item_use > OF(0.0) && !kfs.contains(item_id) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(uad, calc, &fit.ship, max_attr_id).unwrap_or(OF(0.0));
    total_use <= max
}

fn validate_verbose_fitting<'a>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    item_ids: impl ExactSizeIterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
    max_attr_id: &EAttrId,
) -> Option<SolValResFail> {
    let mut total_use = OF(0.0);
    let mut users = Vec::with_capacity(item_ids.len());
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val_simple(uad, item_id, use_attr_id) {
            Some(item_use) => item_use,
            None => continue,
        };
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_id) {
            users.push(SolValResItemInfo {
                item_id: *item_id,
                used: item_use,
            });
        }
    }
    if users.is_empty() {
        return None;
    }
    let total_use = round(total_use, 2);
    let max = get_max_resource(uad, calc, &fit.ship, max_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(SolValResFail {
        used: total_use,
        max,
        users,
    })
}
fn validate_verbose_other<'a>(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    items: impl ExactSizeIterator<Item = (&'a SolItemId, &'a AttrVal)>,
    max_attr_id: &EAttrId,
) -> Option<SolValResFail> {
    let mut total_use = OF(0.0);
    let mut users = Vec::with_capacity(items.len());
    for (item_id, &item_use) in items {
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_id) {
            users.push(SolValResItemInfo {
                item_id: *item_id,
                used: item_use,
            });
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(uad, calc, &fit.ship, max_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(SolValResFail {
        used: total_use,
        max,
        users,
    })
}
