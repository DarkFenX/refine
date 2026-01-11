use std::collections::HashMap;

use super::shared::get_max_resource;
use crate::{
    num::Value,
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UFit, UItemId},
    util::RSet,
};

pub struct ValResFail {
    /// How much resource is used by all of its consumers.
    pub used: Value,
    /// Max available resource (e.g. amount of CPU produced by ship).
    pub max: Option<Value>,
    /// Map between consumer item IDs and amount consumed.
    pub users: HashMap<ItemId, Value>,
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
            self.mods_svcs_online.iter().copied(),
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
            self.mods_svcs_online.iter().copied(),
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
            self.rigs_offline_calibration.iter().map(|(k, v)| (*k, *v)),
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
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_volume.iter().map(|(k, v)| (*k, v.into_value())),
            ctx.ac().drone_capacity,
        )
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
            self.drones_online_bandwidth.iter().map(|(k, v)| (*k, *v)),
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
        validate_fast_other(
            kfs,
            ctx,
            calc,
            fit,
            self.fighters_volume.iter().map(|(k, v)| (*k, v.into_value())),
            ctx.ac().ftr_capacity,
        )
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
            self.mods_svcs_online.iter().copied(),
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
            self.mods_svcs_online.iter().copied(),
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
            self.rigs_offline_calibration.iter().map(|(k, v)| (*k, *v)),
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
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.drones_volume.iter().map(|(k, v)| (*k, v.into_value())),
            ctx.ac().drone_capacity,
        )
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
            self.drones_online_bandwidth.iter().map(|(k, v)| (*k, *v)),
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
        validate_verbose_other(
            kfs,
            ctx,
            calc,
            fit,
            self.fighters_volume.iter().map(|(k, v)| (*k, v.into_value())),
            ctx.ac().ftr_capacity,
        )
    }
}

fn validate_fast_fitting(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = UItemId>,
    use_attr_rid: Option<RAttrId>,
    max_attr_rid: Option<RAttrId>,
) -> bool {
    let mut total_use = Value::ZERO;
    let mut force_pass = true;
    for item_uid in items {
        let item_use = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, use_attr_rid, Value::ZERO)
            .unwrap();
        if force_pass && item_use > Value::FLOAT_TOLERANCE && !kfs.contains(&item_uid) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_rid).unwrap_or(Value::ZERO);
    total_use.rounded_to_digits(2) <= max
}
fn validate_fast_other(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = (UItemId, Value)>,
    max_attr_rid: Option<RAttrId>,
) -> bool {
    let mut total_use = Value::ZERO;
    let mut force_pass = true;
    for (item_uid, item_use) in items {
        if force_pass && item_use > Value::FLOAT_TOLERANCE && !kfs.contains(&item_uid) {
            force_pass = false;
        }
        total_use += item_use;
    }
    if force_pass {
        return true;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_rid).unwrap_or(Value::ZERO);
    total_use <= max
}

fn validate_verbose_fitting(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl ExactSizeIterator<Item = UItemId>,
    use_attr_rid: Option<RAttrId>,
    max_attr_rid: Option<RAttrId>,
) -> Option<ValResFail> {
    let mut total_use = Value::ZERO;
    let mut users = HashMap::with_capacity(items.len());
    for item_uid in items {
        let item_use = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, use_attr_rid, Value::ZERO)
            .unwrap();
        total_use += item_use;
        if item_use > Value::FLOAT_TOLERANCE && !kfs.contains(&item_uid) {
            users.insert(ctx.u_data.items.xid_by_iid(item_uid), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    total_use.round_to_digits(2);
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_rid);
    if total_use <= max.unwrap_or(Value::ZERO) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
fn validate_verbose_other(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl ExactSizeIterator<Item = (UItemId, Value)>,
    max_attr_rid: Option<RAttrId>,
) -> Option<ValResFail> {
    let mut total_use = Value::ZERO;
    let mut users = HashMap::with_capacity(items.len());
    for (item_uid, item_use) in items {
        total_use += item_use;
        if item_use > Value::FLOAT_TOLERANCE && !kfs.contains(&item_uid) {
            users.insert(ctx.u_data.items.xid_by_iid(item_uid), item_use);
        }
    }
    if users.is_empty() {
        return None;
    }
    let max = get_max_resource(ctx, calc, fit.ship, max_attr_rid);
    if total_use <= max.unwrap_or(Value::ZERO) {
        return None;
    }
    Some(ValResFail {
        used: total_use,
        max,
        users,
    })
}
