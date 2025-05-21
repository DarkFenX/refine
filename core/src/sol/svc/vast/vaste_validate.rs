use crate::sol::{
    FitKey,
    reffs::REffs,
    svc::{
        calc::Calc,
        vast::{ValOptions, ValResult, Vast},
    },
    uad::Uad,
};

impl Vast {
    pub(in crate::sol) fn validate_fit_fast(
        &mut self,
        uad: &Uad,
        calc: &mut Calc,
        running_effects: &REffs,
        fit_key: FitKey,
        options: &ValOptions,
    ) -> bool {
        let fit = uad.fits.get(fit_key);
        let fit_data = self.get_fit_data_mut(&fit_key);
        let ship = fit.ship.map(|v| uad.items.get(v).get_ship().unwrap());
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
        if options.service_slot_count.enabled
            && !fit_data.validate_service_slot_count_fast(&options.service_slot_count.kfs, uad, calc, fit)
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
        if options.drone_group.enabled && !fit_data.validate_drone_group_fast(&options.drone_group.kfs) {
            return false;
        }
        if options.fighter_squad_size.enabled
            && !fit_data.validate_fighter_squad_size_fast(&options.fighter_squad_size.kfs)
        {
            return false;
        }
        if options.unlaunchable_drone_slot.enabled
            && !fit_data.validate_unlaunchable_drone_slot_fast(&options.unlaunchable_drone_slot.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.unlaunchable_drone_bandwidth.enabled
            && !fit_data.validate_unlaunchable_drone_bandwidth_fast(
                &options.unlaunchable_drone_bandwidth.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_fighter.enabled
            && !fit_data.validate_unlaunchable_fighter_fast(&options.unlaunchable_fighter.kfs, uad, calc, fit)
        {
            return false;
        }
        if options.unlaunchable_support_fighter.enabled
            && !fit_data.validate_unlaunchable_support_fighter_fast(
                &options.unlaunchable_support_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_light_fighter.enabled
            && !fit_data.validate_unlaunchable_light_fighter_fast(
                &options.unlaunchable_light_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_heavy_fighter.enabled
            && !fit_data.validate_unlaunchable_heavy_fighter_fast(
                &options.unlaunchable_heavy_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_standup_support_fighter.enabled
            && !fit_data.validate_unlaunchable_standup_support_fighter_fast(
                &options.unlaunchable_standup_support_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_standup_light_fighter.enabled
            && !fit_data.validate_unlaunchable_standup_light_fighter_fast(
                &options.unlaunchable_standup_light_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_standup_heavy_fighter.enabled
            && !fit_data.validate_unlaunchable_standup_heavy_fighter_fast(
                &options.unlaunchable_standup_heavy_fighter.kfs,
                uad,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.ship_stance.enabled && !fit_data.validate_ship_stance_fast(&options.ship_stance.kfs, fit, ship) {
            return false;
        }
        if options.overload_skill.enabled && !fit_data.validate_overload_skill_fast(&options.overload_skill.kfs, fit) {
            return false;
        }
        if options.max_type_fitted.enabled && !fit_data.validate_max_type_fitted_fast(&options.max_type_fitted.kfs) {
            return false;
        }
        if options.sec_zone_fitted.enabled
            && !fit_data.validate_sec_zone_fitted_fast(&options.sec_zone_fitted.kfs, uad, calc)
        {
            return false;
        }
        if options.sec_zone_online.enabled && !fit_data.validate_sec_zone_online_fast(&options.sec_zone_online.kfs, uad)
        {
            return false;
        }
        if options.sec_zone_active.enabled
            && !fit_data.validate_sec_zone_active_fast(&options.sec_zone_active.kfs, uad, calc)
        {
            return false;
        }
        if options.sec_zone_unonlineable.enabled
            && !fit_data.validate_sec_zone_unonlineable_fast(&options.sec_zone_unonlineable.kfs, uad)
        {
            return false;
        }
        if options.sec_zone_unactivable.enabled
            && !fit_data.validate_sec_zone_unactivable_fast(&options.sec_zone_unactivable.kfs, uad, calc)
        {
            return false;
        }
        if options.activation_blocked.enabled
            && !fit_data.validate_activation_blocked_fast(&options.activation_blocked.kfs, uad, calc)
        {
            return false;
        }
        if options.item_vs_ship_kind.enabled
            && !fit_data.validate_item_vs_ship_kind_fast(&options.item_vs_ship_kind.kfs)
        {
            return false;
        }
        if options.effect_stopper.enabled
            && !fit_data.validate_effect_stopper_fast(&options.effect_stopper.kfs, running_effects)
        {
            return false;
        }
        if options.assist_immunity.enabled
            && !fit_data.validate_assist_immunity_fast(&options.assist_immunity.kfs, uad, calc)
        {
            return false;
        }
        if options.offense_immunity.enabled
            && !fit_data.validate_offense_immunity_fast(&options.offense_immunity.kfs, uad, calc)
        {
            return false;
        }
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &mut self,
        uad: &Uad,
        calc: &mut Calc,
        running_effects: &REffs,
        fit_key: FitKey,
        options: &ValOptions,
    ) -> ValResult {
        let fit = uad.fits.get(fit_key);
        let fit_data = self.get_fit_data_mut(&fit_key);
        let ship = fit.ship.map(|v| uad.items.get(v).get_ship().unwrap());
        let mut result = ValResult::new();
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
        if options.service_slot_count.enabled {
            result.service_slot_count =
                fit_data.validate_service_slot_count_verbose(&options.service_slot_count.kfs, uad, calc, fit);
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
            result.implant_slot_index =
                fit_data.validate_implant_slot_index_verbose(&options.implant_slot_index.kfs, uad);
        }
        if options.booster_slot_index.enabled {
            result.booster_slot_index =
                fit_data.validate_booster_slot_index_verbose(&options.booster_slot_index.kfs, uad);
        }
        if options.subsystem_slot_index.enabled {
            result.subsystem_slot_index =
                fit_data.validate_subsystem_slot_index_verbose(&options.subsystem_slot_index.kfs, uad);
        }
        if options.ship_limit.enabled {
            result.ship_limit = fit_data.validate_ship_limit_verbose(&options.ship_limit.kfs, uad, ship);
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
            result.rig_size = fit_data.validate_rig_size_verbose(&options.rig_size.kfs, uad, ship);
        }
        if options.skill_reqs.enabled {
            result.skill_reqs = fit_data.validate_skill_reqs_verbose(&options.skill_reqs.kfs, uad);
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
            result.capital_module = fit_data.validate_capital_module_verbose(&options.capital_module.kfs, uad, ship);
        }
        if options.not_loaded_item.enabled {
            result.not_loaded_item = fit_data.validate_not_loaded_item_verbose(&options.not_loaded_item.kfs, uad);
        }
        if options.module_state.enabled {
            result.module_state = fit_data.validate_module_state_verbose(&options.module_state.kfs, uad);
        }
        if options.item_kind.enabled {
            result.item_kind = fit_data.validate_item_kind_verbose(&options.item_kind.kfs, uad);
        }
        if options.drone_group.enabled {
            result.drone_group = fit_data.validate_drone_group_verbose(&options.drone_group.kfs, uad);
        }
        if options.fighter_squad_size.enabled {
            result.fighter_squad_size =
                fit_data.validate_fighter_squad_size_verbose(&options.fighter_squad_size.kfs, uad);
        }
        if options.unlaunchable_drone_slot.enabled {
            result.unlaunchable_drone_slot =
                fit_data.validate_unlaunchable_drone_slot_verbose(&options.unlaunchable_drone_slot.kfs, uad, calc, fit);
        }
        if options.unlaunchable_drone_bandwidth.enabled {
            result.unlaunchable_drone_bandwidth = fit_data.validate_unlaunchable_drone_bandwidth_verbose(
                &options.unlaunchable_drone_bandwidth.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.unlaunchable_fighter.enabled {
            result.unlaunchable_fighter =
                fit_data.validate_unlaunchable_fighter_verbose(&options.unlaunchable_fighter.kfs, uad, calc, fit);
        }
        if options.unlaunchable_support_fighter.enabled {
            result.unlaunchable_support_fighter = fit_data.validate_unlaunchable_support_fighter_verbose(
                &options.unlaunchable_support_fighter.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.unlaunchable_light_fighter.enabled {
            result.unlaunchable_light_fighter = fit_data.validate_unlaunchable_light_fighter_verbose(
                &options.unlaunchable_light_fighter.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.unlaunchable_heavy_fighter.enabled {
            result.unlaunchable_heavy_fighter = fit_data.validate_unlaunchable_heavy_fighter_verbose(
                &options.unlaunchable_heavy_fighter.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.unlaunchable_standup_support_fighter.enabled {
            result.unlaunchable_standup_support_fighter = fit_data
                .validate_unlaunchable_standup_support_fighter_verbose(
                    &options.unlaunchable_standup_support_fighter.kfs,
                    uad,
                    calc,
                    fit,
                );
        }
        if options.unlaunchable_standup_light_fighter.enabled {
            result.unlaunchable_standup_light_fighter = fit_data.validate_unlaunchable_standup_light_fighter_verbose(
                &options.unlaunchable_standup_light_fighter.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.unlaunchable_standup_heavy_fighter.enabled {
            result.unlaunchable_standup_heavy_fighter = fit_data.validate_unlaunchable_standup_heavy_fighter_verbose(
                &options.unlaunchable_standup_heavy_fighter.kfs,
                uad,
                calc,
                fit,
            );
        }
        if options.ship_stance.enabled {
            result.ship_stance = fit_data.validate_ship_stance_verbose(&options.ship_stance.kfs, uad, fit, ship);
        }
        if options.overload_skill.enabled {
            result.overload_skill = fit_data.validate_overload_skill_verbose(&options.overload_skill.kfs, uad, fit);
        }
        if options.max_type_fitted.enabled {
            result.max_type_fitted = fit_data.validate_max_type_fitted_verbose(&options.max_type_fitted.kfs, uad);
        }
        if options.sec_zone_fitted.enabled {
            result.sec_zone_fitted = fit_data.validate_sec_zone_fitted_verbose(&options.sec_zone_fitted.kfs, uad, calc);
        }
        if options.sec_zone_online.enabled {
            result.sec_zone_online = fit_data.validate_sec_zone_online_verbose(&options.sec_zone_online.kfs, uad);
        }
        if options.sec_zone_active.enabled {
            result.sec_zone_active = fit_data.validate_sec_zone_active_verbose(&options.sec_zone_active.kfs, uad, calc);
        }
        if options.sec_zone_unonlineable.enabled {
            result.sec_zone_unonlineable =
                fit_data.validate_sec_zone_unonlineable_verbose(&options.sec_zone_unonlineable.kfs, uad);
        }
        if options.sec_zone_unactivable.enabled {
            result.sec_zone_unactivable =
                fit_data.validate_sec_zone_unactivable_verbose(&options.sec_zone_unactivable.kfs, uad, calc);
        }
        if options.activation_blocked.enabled {
            result.activation_blocked =
                fit_data.validate_activation_blocked_verbose(&options.activation_blocked.kfs, uad, calc);
        }
        if options.item_vs_ship_kind.enabled {
            result.item_vs_ship_kind =
                fit_data.validate_item_vs_ship_kind_verbose(&options.item_vs_ship_kind.kfs, uad, fit);
        }
        if options.effect_stopper.enabled {
            result.effect_stopper =
                fit_data.validate_effect_stopper_verbose(&options.effect_stopper.kfs, uad, running_effects);
        }
        if options.assist_immunity.enabled {
            result.assist_immunity = fit_data.validate_assist_immunity_verbose(&options.assist_immunity.kfs, uad, calc);
        }
        if options.offense_immunity.enabled {
            result.offense_immunity =
                fit_data.validate_offense_immunity_verbose(&options.offense_immunity.kfs, uad, calc);
        }
        result
    }
}
