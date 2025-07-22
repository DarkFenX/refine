use std::collections::HashMap;

use super::shared::get_max_resource;
use crate::{
    ac, ad,
    def::{AttrVal, ItemId, OF},
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    uad::{UadFit, UadItemKey},
    util::{RSet, round},
};

pub struct ValResFail {
    /// How much resource is used by all of its consumers.
    pub used: AttrVal,
    /// Max available resource (e.g. amount of CPU produced by ship).
    pub max: Option<AttrVal>,
    /// Map between consumer item IDs and amount consumed.
    pub users: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_cpu_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::svc::vast) fn validate_powergrid_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::svc::vast) fn validate_calibration_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.iter(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_volume.iter(),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ac::attrs::FTR_CAPACITY,
        )
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_cpu_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::svc::vast) fn validate_powergrid_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::svc::vast) fn validate_calibration_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.iter(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_volume.iter(),
            &ac::attrs::DRONE_CAPACITY,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.fighters_volume.iter(),
            &ac::attrs::FTR_CAPACITY,
        )
    }
}

fn validate_fast_fitting<'a>(
    kfs: &RSet<UadItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl Iterator<Item = &'a UadItemKey>,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for item_key in items {
        let item_use = calc.get_item_attr_val_extra_opt(ctx, *item_key, use_a_attr_id).unwrap();
        if force_pass && item_use > OF(0.0) && !kfs.contains(item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    round(total_use, 2) <= max
}
fn validate_fast_other<'a>(
    kfs: &RSet<UadItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl Iterator<Item = (&'a UadItemKey, &'a ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for (item_key, &item_use) in items {
        if force_pass && item_use > OF(0.0) && !kfs.contains(item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_a_attr_id).unwrap_or(OF(0.0));
    total_use <= max
}

fn validate_verbose_fitting<'a>(
    kfs: &RSet<UadItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl ExactSizeIterator<Item = &'a UadItemKey>,
    use_a_attr_id: &ad::AAttrId,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::with_capacity(items.len());
    for item_key in items {
        let item_use = calc.get_item_attr_val_extra_opt(ctx, *item_key, use_a_attr_id).unwrap();
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_key) {
            users.insert(ctx.uad.items.id_by_key(*item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let total_use = round(total_use, 2);
    let max = get_max_resource(ctx, calc, fit.ship, max_a_attr_id);
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
    kfs: &RSet<UadItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl ExactSizeIterator<Item = (&'a UadItemKey, &'a ad::AAttrVal)>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::with_capacity(items.len());
    for (item_key, &item_use) in items {
        total_use += item_use;
        if item_use > OF(0.0) && !kfs.contains(item_key) {
            users.insert(ctx.uad.items.id_by_key(*item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_a_attr_id);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
