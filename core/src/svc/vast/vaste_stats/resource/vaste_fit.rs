use super::stat::StatResource;
use crate::{
    num::Value,
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{UFit, UItemId},
};

impl VastFitData {
    // Public methods
    pub(in crate::svc) fn get_stat_cpu(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter().copied(),
            ctx.ac().cpu,
            ctx.ac().cpu_output,
        )
    }
    pub(in crate::svc) fn get_stat_powergrid(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter().copied(),
            ctx.ac().power,
            ctx.ac().power_output,
        )
    }
    pub(in crate::svc) fn get_stat_calibration(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.values().copied(),
            ctx.ac().upgrade_capacity,
        )
    }
    pub(in crate::svc) fn get_stat_drone_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.drones_volume.values().map(|v| v.into_value()),
            ctx.ac().drone_capacity,
        )
    }
    pub(in crate::svc) fn get_stat_drone_bandwidth(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.values().copied(),
            ctx.ac().drone_bandwidth,
        )
    }
    pub(in crate::svc) fn get_stat_fighter_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatResource {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.fighters_volume.values().map(|v| v.into_value()),
            ctx.ac().ftr_capacity,
        )
    }
}

fn get_resource_stats_fitting(
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = UItemId>,
    use_attr_rid: Option<RAttrId>,
    output_attr_rid: Option<RAttrId>,
) -> StatResource {
    let output = calc.get_oitem_oattr_afb_oextra(ctx, fit.ship, output_attr_rid, Value::ZERO);
    let used: Value = items
        .filter_map(|item_uid| calc.get_item_oattr_oextra(ctx, item_uid, use_attr_rid))
        .sum();
    // Round possible float errors despite individual use values being rounded
    StatResource {
        used: used.rounded_to_digits(2),
        output,
    }
}
fn get_resource_stats_other(
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items_use: impl Iterator<Item = Value>,
    output_attr_rid: Option<RAttrId>,
) -> StatResource {
    let output = calc.get_oitem_oattr_afb_oextra(ctx, fit.ship, output_attr_rid, Value::ZERO);
    let used = items_use.sum();
    StatResource { used, output }
}
