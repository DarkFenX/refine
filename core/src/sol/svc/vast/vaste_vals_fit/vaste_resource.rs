use crate::{
    defs::{AttrVal, EAttrId, OF, SolItemId},
    ec,
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatRes, SolVastFitData},
        },
        uad::{SolUad, fit::SolFit},
    },
    util::{StSet, round},
};

pub struct SolValResFail {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
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
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_fitting(
            uad,
            calc,
            fit,
            kfs,
            self.mods_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_fitting(
            uad,
            calc,
            fit,
            kfs,
            self.mods_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_other(
            uad,
            calc,
            fit,
            kfs,
            self.rigs_rigslot_calibration.iter(),
            &ec::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_other(
            uad,
            calc,
            fit,
            kfs,
            self.drones_volume.iter(),
            &ec::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_other(
            uad,
            calc,
            fit,
            kfs,
            self.drones_online_bandwidth.iter(),
            &ec::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        validate_fast_other(
            uad,
            calc,
            fit,
            kfs,
            self.fighters_volume.iter(),
            &ec::attrs::FTR_CAPACITY,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_fitting(
            uad,
            calc,
            fit,
            kfs,
            self.mods_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_fitting(
            uad,
            calc,
            fit,
            kfs,
            self.mods_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            uad,
            calc,
            fit,
            kfs,
            self.rigs_rigslot_calibration.iter(),
            &ec::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            uad,
            calc,
            fit,
            kfs,
            self.drones_volume.iter(),
            &ec::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            uad,
            calc,
            fit,
            kfs,
            self.drones_online_bandwidth.iter(),
            &ec::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        validate_verbose_other(
            uad,
            calc,
            fit,
            kfs,
            self.fighters_volume.iter(),
            &ec::attrs::FTR_CAPACITY,
        )
    }
}

fn validate_fast_fitting<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    kfs: &StSet<SolItemId>,
    item_ids: impl Iterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
    output_attr_id: &EAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val(uad, item_id, use_attr_id) {
            Ok(attr_val) => attr_val.extra,
            Err(_) => continue,
        };
        if force_pass && item_use > OF(0.0) && !kfs.contains(item_id) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let output = get_item_attr(uad, calc, &fit.ship, output_attr_id).unwrap_or(OF(0.0));
    round(total_use, 2) <= output
}
fn validate_fast_other<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    kfs: &StSet<SolItemId>,
    items: impl Iterator<Item = (&'a SolItemId, &'a AttrVal)>,
    output_attr_id: &EAttrId,
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
    let output = get_item_attr(uad, calc, &fit.ship, output_attr_id).unwrap_or(OF(0.0));
    force_pass || total_use <= output
}

fn validate_verbose_fitting<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    kfs: &StSet<SolItemId>,
    item_ids: impl ExactSizeIterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
    output_attr_id: &EAttrId,
) -> Option<SolValResFail> {
    let mut total_use = OF(0.0);
    let mut users = Vec::with_capacity(item_ids.len());
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val(uad, item_id, use_attr_id) {
            Ok(attr_val) => attr_val.extra,
            Err(_) => continue,
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
    let output = get_item_attr(uad, calc, &fit.ship, output_attr_id);
    if total_use <= output.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(SolValResFail {
        used: total_use,
        output,
        users,
    })
}
fn validate_verbose_other<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    kfs: &StSet<SolItemId>,
    items: impl ExactSizeIterator<Item = (&'a SolItemId, &'a AttrVal)>,
    output_attr_id: &EAttrId,
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
    let output = get_item_attr(uad, calc, &fit.ship, output_attr_id);
    if total_use <= output.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(SolValResFail {
        used: total_use,
        output,
        users,
    })
}

fn get_item_attr(uad: &SolUad, calc: &mut SolCalc, item_id: &Option<SolItemId>, attr_id: &EAttrId) -> Option<AttrVal> {
    match item_id {
        Some(item_id) => match calc.get_item_attr_val(uad, item_id, attr_id) {
            Ok(attr_val) => Some(attr_val.extra),
            Err(_) => None,
        },
        None => None,
    }
}
