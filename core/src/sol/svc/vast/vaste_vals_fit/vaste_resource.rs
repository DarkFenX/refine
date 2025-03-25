use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::Fit},
    },
    util::{StSet, round},
};

use super::shared::get_max_resource;

pub struct ValResFail {
    pub used: AttrVal,
    pub max: Option<AttrVal>,
    pub users: Vec<ValResItemInfo>,
}

pub struct ValResItemInfo {
    pub item_id: ItemId,
    pub used: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.rigs_rigslot_calibration.iter(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_volume.iter(),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> bool {
        validate_fast_other(
            kfs,
            uad,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ac::attrs::FTR_CAPACITY,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.rigs_rigslot_calibration.iter(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_volume.iter(),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        kfs: &StSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            uad,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ac::attrs::FTR_CAPACITY,
        )
    }
}

fn validate_fast_fitting<'a>(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    item_ids: impl Iterator<Item = &'a ItemId>,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val_simple(uad, item_id, use_a_attr_id) {
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
    let max = get_max_resource(uad, calc, &fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    round(total_use, 2) <= max
}
fn validate_fast_other<'a>(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    items: impl Iterator<Item = (&'a ItemId, &'a ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
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
    let max = get_max_resource(uad, calc, &fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    total_use <= max
}

fn validate_verbose_fitting<'a>(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    item_ids: impl ExactSizeIterator<Item = &'a ItemId>,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = Vec::with_capacity(item_ids.len());
    for item_id in item_ids {
        let item_use = match calc.get_item_attr_val_simple(uad, item_id, use_a_attr_id) {
            Some(item_use) => item_use,
            None => continue,
        };
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_id) {
            users.push(ValResItemInfo {
                item_id: *item_id,
                used: item_use,
            });
        }
    }
    if users.is_empty() {
        return None;
    }
    let total_use = round(total_use, 2);
    let max = get_max_resource(uad, calc, &fit.ship, max_a_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
fn validate_verbose_other<'a>(
    kfs: &StSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    items: impl ExactSizeIterator<Item = (&'a ItemId, &'a ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = Vec::with_capacity(items.len());
    for (item_id, &item_use) in items {
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_id) {
            users.push(ValResItemInfo {
                item_id: *item_id,
                used: item_use,
            });
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(uad, calc, &fit.ship, max_a_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
