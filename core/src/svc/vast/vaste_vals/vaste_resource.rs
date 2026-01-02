use std::collections::HashMap;

use super::shared::get_max_resource;
use crate::{
    ad::AAttrVal,
    def::{AttrVal, ItemId, OF},
    rd::RAttrKey,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{UFit, UItemId},
    util::{FLOAT_TOLERANCE, RSet, round},
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
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().cpu,
            ctx.ac().cpu_output,
        )
    }
    pub(in crate::svc::vast) fn validate_powergrid_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().power,
            ctx.ac().power_output,
        )
    }
    pub(in crate::svc::vast) fn validate_calibration_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.iter(),
            ctx.ac().upgrade_capacity,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_other(kfs, ctx, calc, fit, self.drones_volume.iter(), ctx.ac().drone_capacity)
    }
    pub(in crate::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            ctx.ac().drone_bandwidth,
        )
    }
    pub(in crate::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_other(kfs, ctx, calc, fit, self.fighters_volume.iter(), ctx.ac().ftr_capacity)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_cpu_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().cpu,
            ctx.ac().cpu_output,
        )
    }
    pub(in crate::svc::vast) fn validate_powergrid_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_fitting(
            kfs,
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().power,
            ctx.ac().power_output,
        )
    }
    pub(in crate::svc::vast) fn validate_calibration_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.iter(),
            ctx.ac().upgrade_capacity,
        )
    }
    pub(in crate::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(kfs, ctx, calc, fit, self.drones_volume.iter(), ctx.ac().drone_capacity)
    }
    pub(in crate::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.iter(),
            ctx.ac().drone_bandwidth,
        )
    }
    pub(in crate::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValResFail> {
        validate_verbose_other(kfs, ctx, calc, fit, self.fighters_volume.iter(), ctx.ac().ftr_capacity)
    }
}

fn validate_fast_fitting<'a>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = &'a UItemId>,
    use_attr_key: Option<RAttrKey>,
    max_attr_key: Option<RAttrKey>,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for &item_key in items {
        let item_use = calc
            .get_item_oattr_afb_oextra(ctx, item_key, use_attr_key, OF(0.0))
            .unwrap();
        if force_pass && item_use > FLOAT_TOLERANCE && !kfs.contains(&item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_key).unwrap_or(OF(0.0));
    round(total_use, 2) <= max
}
fn validate_fast_other<'a>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = (&'a UItemId, &'a AAttrVal)>,
    max_attr_key: Option<RAttrKey>,
) -> bool {
    let mut total_use = OF(0.0);
    let mut force_pass = true;
    for (item_key, &item_use) in items {
        if force_pass && item_use > FLOAT_TOLERANCE && !kfs.contains(item_key) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_key).unwrap_or(OF(0.0));
    total_use <= max
}

fn validate_verbose_fitting<'a>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl ExactSizeIterator<Item = &'a UItemId>,
    use_attr_key: Option<RAttrKey>,
    max_attr_key: Option<RAttrKey>,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::with_capacity(items.len());
    for &item_key in items {
        let item_use = calc
            .get_item_oattr_afb_oextra(ctx, item_key, use_attr_key, OF(0.0))
            .unwrap();
        total_use += item_use;
        if item_use > FLOAT_TOLERANCE && !kfs.contains(&item_key) {
            users.insert(ctx.u_data.items.ext_id_by_int_id(item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let total_use = round(total_use, 2);
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_key);
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
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl ExactSizeIterator<Item = (&'a UItemId, &'a AAttrVal)>,
    max_attr_key: Option<RAttrKey>,
) -> Option<ValResFail> {
    let mut total_use = OF(0.0);
    let mut users = HashMap::with_capacity(items.len());
    for (item_key, &item_use) in items {
        total_use += item_use;
        if item_use > FLOAT_TOLERANCE && !kfs.contains(item_key) {
            users.insert(ctx.u_data.items.ext_id_by_int_id(*item_key), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_key);
    if total_use <= max.unwrap_or(OF(0.0)) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
