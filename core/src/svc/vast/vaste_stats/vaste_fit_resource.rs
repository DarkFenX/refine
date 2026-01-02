use crate::{
    def::{AttrVal, OF},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{UFit, UItemId},
    util::round,
};

pub struct StatRes {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
}

impl VastFitData {
    // Public methods
    pub(in crate::svc) fn get_stat_cpu(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().cpu,
            ctx.ac().cpu_output,
        )
    }
    pub(in crate::svc) fn get_stat_powergrid(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            ctx.ac().power,
            ctx.ac().power_output,
        )
    }
    pub(in crate::svc) fn get_stat_calibration(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.values(),
            ctx.ac().upgrade_capacity,
        )
    }
    pub(in crate::svc) fn get_stat_drone_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_other(ctx, calc, fit, self.drones_volume.values(), ctx.ac().drone_capacity)
    }
    pub(in crate::svc) fn get_stat_drone_bandwidth(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.values(),
            ctx.ac().drone_bandwidth,
        )
    }
    pub(in crate::svc) fn get_stat_fighter_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatRes {
        get_resource_stats_other(ctx, calc, fit, self.fighters_volume.values(), ctx.ac().ftr_capacity)
    }
}

fn get_resource_stats_fitting<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items: impl Iterator<Item = &'a UItemId>,
    use_attr_key: Option<RAttrId>,
    output_attr_key: Option<RAttrId>,
) -> StatRes {
    let output = calc.get_oitem_oattr_afb_oextra(ctx, fit.ship, output_attr_key, OF(0.0));
    let used = items
        .filter_map(|&item_key| calc.get_item_oattr_oextra(ctx, item_key, use_attr_key))
        .sum();
    // Round possible float errors despite individual use values being rounded
    StatRes {
        used: round(used, 2),
        output,
    }
}
fn get_resource_stats_other<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UFit,
    items_use: impl Iterator<Item = &'a AttrVal>,
    output_attr_key: Option<RAttrId>,
) -> StatRes {
    let output = calc.get_oitem_oattr_afb_oextra(ctx, fit.ship, output_attr_key, OF(0.0));
    let used = items_use.sum();
    StatRes { used, output }
}
