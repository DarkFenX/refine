use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemKey,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::Fit},
    },
    util::round,
};

pub struct StatRes {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
}

impl VastFitData {
    // Public methods
    pub(in crate::sol::svc::vast) fn get_stats_cpu(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatRes {
        get_resource_stats_fitting(
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::CPU,
            &ac::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_powergrid(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatRes {
        get_resource_stats_fitting(
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ac::attrs::POWER,
            &ac::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_calibration(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatRes {
        get_resource_stats_other(
            uad,
            calc,
            fit,
            self.rigs_offline_calibration.values(),
            &ac::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_drone_bay_volume(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatRes {
        get_resource_stats_other(uad, calc, fit, self.drones_volume.values(), &ac::attrs::DRONE_CAPACITY)
    }
    pub(in crate::sol::svc::vast) fn get_stats_drone_bandwidth(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatRes {
        get_resource_stats_other(
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.values(),
            &ac::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_fighter_bay_volume(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatRes {
        get_resource_stats_other(uad, calc, fit, self.fighters_volume.values(), &ac::attrs::FTR_CAPACITY)
    }
}

fn get_resource_stats_fitting<'a>(
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    items: impl Iterator<Item = &'a ItemKey>,
    use_a_attr_id: &ad::AAttrId,
    output_a_attr_id: &ad::AAttrId,
) -> StatRes {
    let output = calc.get_item_attr_val_extra_opt(uad, fit.ship, output_a_attr_id);
    let used = items
        .filter_map(|item_key| calc.get_item_attr_val_extra(uad, *item_key, use_a_attr_id))
        .sum();
    // Round possible float errors despite individual use values being rounded
    StatRes {
        used: round(used, 2),
        output,
    }
}
fn get_resource_stats_other<'a>(
    uad: &Uad,
    calc: &mut Calc,
    fit: &Fit,
    items_use: impl Iterator<Item = &'a AttrVal>,
    output_a_attr_id: &ad::AAttrId,
) -> StatRes {
    let output = calc.get_item_attr_val_extra_opt(uad, fit.ship, output_a_attr_id);
    let used = items_use.sum();
    StatRes { used, output }
}
