use crate::{
    ac, ad,
    def::AttrVal,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    uad::{UadFit, UadItemKey},
    util::round,
};

pub struct StatRes {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
}

impl VastFitData {
    // Public methods
    pub(in crate::svc) fn get_stat_cpu(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::svc) fn get_stat_powergrid(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_fitting(
            ctx,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::svc) fn get_stat_calibration(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.rigs_offline_calibration.values(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::svc) fn get_stat_drone_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_other(ctx, calc, fit, self.drones_volume.values(), &ac::attrs::DRONE_CAPACITY)
    }
    pub(in crate::svc) fn get_stat_drone_bandwidth(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_other(
            ctx,
            calc,
            fit,
            self.drones_online_bandwidth.values(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::svc) fn get_stat_fighter_bay_volume(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UadFit) -> StatRes {
        get_resource_stats_other(ctx, calc, fit, self.fighters_volume.values(), &ac::attrs::FTR_CAPACITY)
    }
}

fn get_resource_stats_fitting<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    fit: &UadFit,
    items: impl Iterator<Item = &'a UadItemKey>,
    use_a_attr_id: &ad::AAttrId,
    output_a_attr_id: &ad::AAttrId,
) -> StatRes {
    let output = calc.get_item_attr_val_extra_opt_opt(ctx, fit.ship, output_a_attr_id);
    let used = items
        .filter_map(|item_key| calc.get_item_attr_val_extra_opt(ctx, *item_key, use_a_attr_id))
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
    fit: &UadFit,
    items_use: impl Iterator<Item = &'a AttrVal>,
    output_a_attr_id: &ad::AAttrId,
) -> StatRes {
    let output = calc.get_item_attr_val_extra_opt_opt(ctx, fit.ship, output_a_attr_id);
    let used = items_use.sum();
    StatRes { used, output }
}
