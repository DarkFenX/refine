use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolValResult, SolVast},
    },
    uad::{SolUad, fit::SolFit},
};

impl SolVast {
    pub(in crate::sol) fn validate_fit_fast(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: &SolValOptions,
    ) -> bool {
        let ship = fit.ship.map(|v| uad.items.get_item(&v).unwrap().get_ship().unwrap());
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data_mut(&fit.id).unwrap();
        // Order of validations matters here; the faster validation and the more likely it is to
        // fail, the closer to top it should be
        if options.cpu.enabled && !fit_data.validate_cpu_fast(&options.cpu.kfs, uad, calc, fit) {
            return false;
        }
        if options.powergrid.enabled && !fit_data.validate_powergrid_fast(&options.powergrid.kfs, uad, calc, fit) {
            return false;
        }
        if options.calibration.enabled && !fit_data.validate_calibration_fast(&options.calibration.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.drone_bay_volume.enabled
            && !fit_data.validate_drone_bay_volume_fast(&options.drone_bay_volume.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.drone_bandwidth.enabled
            && !fit_data.validate_drone_bandwidth_fast(&options.drone_bandwidth.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.fighter_bay_volume.enabled
            && !fit_data.validate_fighter_bay_volume_fast(&options.fighter_bay_volume.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.rig_slot_count.enabled
            && !fit_data.validate_rig_slot_count_fast(&options.rig_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.subsystem_slot_count.enabled
            && !fit_data.validate_subsystem_slot_count_fast(&options.subsystem_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.launched_drone_count.enabled
            && !fit_data.validate_launched_drone_count_fast(&options.launched_drone_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.launched_fighter_count.enabled
            && !fit_data.validate_launched_fighter_count_fast(&options.launched_fighter_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.launched_support_fighter_count.enabled
            && !fit_data.validate_launched_support_fighter_count_fast(
                &options.launched_support_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_light_fighter_count.enabled
            && !fit_data.validate_launched_light_fighter_count_fast(
                &options.launched_light_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_heavy_fighter_count.enabled
            && !fit_data.validate_launched_heavy_fighter_count_fast(
                &options.launched_heavy_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_standup_support_fighter_count.enabled
            && !fit_data.validate_launched_standup_support_fighter_count_fast(
                &options.launched_standup_support_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_standup_light_fighter_count.enabled
            && !fit_data.validate_launched_standup_light_fighter_count_fast(
                &options.launched_standup_light_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_standup_heavy_fighter_count.enabled
            && !fit_data.validate_launched_standup_heavy_fighter_count_fast(
                &options.launched_standup_heavy_fighter_count.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.turret_slot_count.enabled
            && !fit_data.validate_turret_slot_count_fast(&options.turret_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.launcher_slot_count.enabled
            && !fit_data.validate_launcher_slot_count_fast(&options.launcher_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.high_slot_count.enabled
            && !fit_data.validate_high_slot_count_fast(&options.high_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.mid_slot_count.enabled
            && !fit_data.validate_mid_slot_count_fast(&options.mid_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.low_slot_count.enabled
            && !fit_data.validate_low_slot_count_fast(&options.low_slot_count.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.implant_slot_index.enabled
            && !fit_data.validate_implant_slot_index_fast(&options.implant_slot_index.kfs)
        {
            return false;
        }
        if options.booster_slot_index.enabled
            && !fit_data.validate_booster_slot_index_fast(&options.booster_slot_index.kfs)
        {
            return false;
        }
        if options.subsystem_slot_index.enabled
            && !fit_data.validate_subsystem_slot_index_fast(&options.subsystem_slot_index.kfs)
        {
            return false;
        }
        if options.ship_limit.enabled && !fit_data.validate_ship_limit_fast(&options.ship_limit.kfs, ship) {
            return false;
        }
        if options.max_group_fitted.enabled
            && !fit_data.validate_max_group_fitted_fast(&options.max_group_fitted.kfs, uad, calc)
        {
            return false;
        }
        if options.max_group_online.enabled
            && !fit_data.validate_max_group_online_fast(&options.max_group_online.kfs, uad, calc)
        {
            return false;
        }
        if options.max_group_active.enabled
            && !fit_data.validate_max_group_active_fast(&options.max_group_active.kfs, uad, calc)
        {
            return false;
        }
        if options.rig_size.enabled && !fit_data.validate_rig_size_fast(&options.rig_size.kfs, ship) {
            return false;
        }
        if options.skill_reqs.enabled && !fit_data.validate_skill_reqs_fast(&options.skill_reqs.kfs) {
            return false;
        }
        if options.charge_group.enabled && !fit_data.validate_charge_group_fast(&options.charge_group.kfs, uad) {
            return false;
        }
        if options.charge_size.enabled && !fit_data.validate_charge_size_fast(&options.charge_size.kfs, uad) {
            return false;
        }
        if options.charge_volume.enabled && !fit_data.validate_charge_volume_fast(&options.charge_volume.kfs, uad) {
            return false;
        }
        if options.capital_module.enabled && !fit_data.validate_capital_module_fast(&options.capital_module.kfs, ship) {
            return false;
        }
        if options.not_loaded_item.enabled && !fit_data.validate_not_loaded_item_fast(&options.not_loaded_item.kfs) {
            return false;
        }
        if options.module_state.enabled && !fit_data.validate_module_state_fast(&options.module_state.kfs) {
            return false;
        }
        if options.item_kind.enabled && !fit_data.validate_item_kind_fast(&options.item_kind.kfs) {
            return false;
        }
        if options.drone_group.enabled && !fit_data.validate_drone_group_fast() {
            return false;
        }
        if options.fighter_count.enabled && !fit_data.validate_fighter_count_fast() {
            return false;
        }
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: &SolValOptions,
    ) -> SolValResult {
        let ship = fit.ship.map(|v| uad.items.get_item(&v).unwrap().get_ship().unwrap());
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data_mut(&fit.id).unwrap();
        let mut result = SolValResult::new();
        if options.cpu.enabled {
            result.cpu = fit_data.validate_cpu_verbose(&options.cpu.kfs, uad, calc, fit);
        }
        if options.powergrid.enabled {
            result.powergrid = fit_data.validate_powergrid_verbose(&options.powergrid.kfs, uad, calc, fit);
        }
        if options.calibration.enabled {
            result.calibration = fit_data.validate_calibration_verbose(&options.calibration.kfs, uad, calc, fit);
        }
        if options.drone_bay_volume.enabled {
            result.drone_bay_volume =
                fit_data.validate_drone_bay_volume_verbose(&options.drone_bay_volume.kfs, uad, calc, fit);
        }
        if options.drone_bandwidth.enabled {
            result.drone_bandwidth =
                fit_data.validate_drone_bandwidth_verbose(&options.drone_bandwidth.kfs, uad, calc, fit);
        }
        if options.fighter_bay_volume.enabled {
            result.fighter_bay_volume =
                fit_data.validate_fighter_bay_volume_verbose(&options.fighter_bay_volume.kfs, uad, calc, fit);
        }
        if options.rig_slot_count.enabled {
            result.rig_slot_count =
                fit_data.validate_rig_slot_count_verbose(&options.rig_slot_count.kfs, uad, calc, fit);
        }
        if options.subsystem_slot_count.enabled {
            result.subsystem_slot_count =
                fit_data.validate_subsystem_slot_count_verbose(&options.subsystem_slot_count.kfs, uad, calc, fit);
        }
        if options.launched_drone_count.enabled {
            result.launched_drone_count =
                fit_data.validate_launched_drone_count_verbose(&options.launched_drone_count.kfs, uad, calc, fit);
        }
        if options.launched_fighter_count.enabled {
            result.launched_fighter_count =
                fit_data.validate_launched_fighter_count_verbose(&options.launched_fighter_count.kfs, uad, calc, fit);
        }
        if options.launched_support_fighter_count.enabled {
            result.launched_support_fighter_count = fit_data.validate_launched_support_fighter_count_verbose(
                &options.launched_support_fighter_count.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.launched_light_fighter_count.enabled {
            result.launched_light_fighter_count = fit_data.validate_launched_light_fighter_count_verbose(
                &options.launched_light_fighter_count.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.launched_heavy_fighter_count.enabled {
            result.launched_heavy_fighter_count = fit_data.validate_launched_heavy_fighter_count_verbose(
                &options.launched_heavy_fighter_count.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.launched_standup_support_fighter_count.enabled {
            result.launched_standup_support_fighter_count = fit_data
                .validate_launched_standup_support_fighter_count_verbose(
                    &options.launched_standup_support_fighter_count.kfs,
                    uad,
                    calc,
                    fit,
                );
        }
        if options.launched_standup_light_fighter_count.enabled {
            result.launched_standup_light_fighter_count = fit_data
                .validate_launched_standup_light_fighter_count_verbose(
                    &options.launched_standup_light_fighter_count.kfs,
                    uad,
                    calc,
                    fit,
                );
        }
        if options.launched_standup_heavy_fighter_count.enabled {
            result.launched_standup_heavy_fighter_count = fit_data
                .validate_launched_standup_heavy_fighter_count_verbose(
                    &options.launched_standup_heavy_fighter_count.kfs,
                    uad,
                    calc,
                    fit,
                );
        }
        if options.turret_slot_count.enabled {
            result.turret_slot_count =
                fit_data.validate_turret_slot_count_verbose(&options.turret_slot_count.kfs, uad, calc, fit);
        }
        if options.launcher_slot_count.enabled {
            result.launcher_slot_count =
                fit_data.validate_launcher_slot_count_verbose(&options.launcher_slot_count.kfs, uad, calc, fit);
        }
        if options.high_slot_count.enabled {
            result.high_slot_count =
                fit_data.validate_high_slot_count_verbose(&options.high_slot_count.kfs, uad, calc, fit);
        }
        if options.mid_slot_count.enabled {
            result.mid_slot_count =
                fit_data.validate_mid_slot_count_verbose(&options.mid_slot_count.kfs, uad, calc, fit);
        }
        if options.low_slot_count.enabled {
            result.low_slot_count =
                fit_data.validate_low_slot_count_verbose(&options.low_slot_count.kfs, uad, calc, fit);
        }
        if options.implant_slot_index.enabled {
            result.implant_slot_index = fit_data.validate_implant_slot_index_verbose(&options.implant_slot_index.kfs);
        }
        if options.booster_slot_index.enabled {
            result.booster_slot_index = fit_data.validate_booster_slot_index_verbose(&options.booster_slot_index.kfs);
        }
        if options.subsystem_slot_index.enabled {
            result.subsystem_slot_index =
                fit_data.validate_subsystem_slot_index_verbose(&options.subsystem_slot_index.kfs);
        }
        if options.ship_limit.enabled {
            result.ship_limit = fit_data.validate_ship_limit_verbose(&options.ship_limit.kfs, ship);
        }
        if options.max_group_fitted.enabled {
            result.max_group_fitted =
                fit_data.validate_max_group_fitted_verbose(&options.max_group_fitted.kfs, uad, calc);
        }
        if options.max_group_online.enabled {
            result.max_group_online =
                fit_data.validate_max_group_online_verbose(&options.max_group_online.kfs, uad, calc);
        }
        if options.max_group_active.enabled {
            result.max_group_active =
                fit_data.validate_max_group_active_verbose(&options.max_group_active.kfs, uad, calc);
        }
        if options.rig_size.enabled {
            result.rig_size = fit_data.validate_rig_size_verbose(&options.rig_size.kfs, ship);
        }
        if options.skill_reqs.enabled {
            result.skill_reqs = fit_data.validate_skill_reqs_verbose(&options.skill_reqs.kfs);
        }
        if options.charge_group.enabled {
            result.charge_group = fit_data.validate_charge_group_verbose(&options.charge_group.kfs, uad);
        }
        if options.charge_size.enabled {
            result.charge_size = fit_data.validate_charge_size_verbose(&options.charge_size.kfs, uad);
        }
        if options.charge_volume.enabled {
            result.charge_volume = fit_data.validate_charge_volume_verbose(&options.charge_volume.kfs, uad);
        }
        if options.capital_module.enabled {
            result.capital_module = fit_data.validate_capital_module_verbose(&options.capital_module.kfs, ship);
        }
        if options.not_loaded_item.enabled {
            result.not_loaded_item = fit_data.validate_not_loaded_item_verbose(&options.not_loaded_item.kfs);
        }
        if options.module_state.enabled {
            result.module_state = fit_data.validate_module_state_verbose(&options.module_state.kfs);
        }
        if options.item_kind.enabled {
            result.item_kind = fit_data.validate_item_kind_verbose(&options.item_kind.kfs);
        }
        if options.drone_group.enabled {
            result.drone_group = fit_data.validate_drone_group_verbose();
        }
        if options.fighter_count.enabled {
            result.fighter_count = fit_data.validate_fighter_count_verbose();
        }
        result
    }
}
