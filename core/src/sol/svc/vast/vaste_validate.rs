use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolValResult, SolVast},
    },
    uad::{fit::SolFit, SolUad},
};

impl SolVast {
    pub(in crate::sol) fn validate_fit_fast(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> bool {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data_mut(&fit.id).unwrap();
        // Order of validations matters here; the faster validation and the more likely it is to
        // fail, the closer to top it should be
        if options.cpu {
            if !fit_data.validate_cpu_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.powergrid {
            if !fit_data.validate_powergrid_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.calibration {
            if !fit_data.validate_calibration_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.dronebay_volume {
            if !fit_data.validate_dronebay_volume_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.drone_bandwidth {
            if !fit_data.validate_drone_bandwidth_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.rig_slots {
            if !fit_data.validate_rig_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.subsystem_slots {
            if !fit_data.validate_subsystem_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_drones {
            if !fit_data.validate_launched_drones_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_fighters {
            if !fit_data.validate_launched_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_support_fighters {
            if !fit_data.validate_launched_support_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_light_fighters {
            if !fit_data.validate_launched_light_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_heavy_fighters {
            if !fit_data.validate_launched_heavy_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_standup_support_fighters {
            if !fit_data.validate_launched_standup_support_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_standup_light_fighters {
            if !fit_data.validate_launched_standup_light_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_standup_heavy_fighters {
            if !fit_data.validate_launched_standup_heavy_fighters_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.turret_slots {
            if !fit_data.validate_turret_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launcher_slots {
            if !fit_data.validate_launcher_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.high_slots {
            if !fit_data.validate_high_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.mid_slots {
            if !fit_data.validate_mid_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.low_slots {
            if !fit_data.validate_low_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.implant_slot_index {
            if !fit_data.validate_implant_slot_index_fast() {
                return false;
            }
        }
        if options.booster_slot_index {
            if !fit_data.validate_booster_slot_index_fast() {
                return false;
            }
        }
        if options.subsystem_slot_index {
            if !fit_data.validate_subsystem_slot_index_fast() {
                return false;
            }
        }
        if options.ship_limit {
            if !fit_data.validate_ship_limit_fast(uad, fit) {
                return false;
            }
        }
        if options.max_group_fitted {
            if !fit_data.validate_max_group_fitted_fast(uad, calc) {
                return false;
            }
        }
        if options.max_group_online {
            if !fit_data.validate_max_group_online_fast(uad, calc) {
                return false;
            }
        }
        if options.max_group_active {
            if !fit_data.validate_max_group_active_fast(uad, calc) {
                return false;
            }
        }
        if options.rig_size {
            if !fit_data.validate_rig_size_fast(uad, fit) {
                return false;
            }
        }
        if options.skill_reqs {
            if !fit_data.validate_skill_reqs_fast() {
                return false;
            }
        }
        if options.charge_group {
            if !fit_data.validate_charge_group_fast(uad) {
                return false;
            }
        }
        if options.charge_size {
            if !fit_data.validate_charge_size_fast(uad) {
                return false;
            }
        }
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &mut self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> SolValResult {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data_mut(&fit.id).unwrap();
        let mut result = SolValResult::new();
        if options.cpu {
            result.cpu = fit_data.validate_cpu_verbose(uad, calc, fit);
        }
        if options.powergrid {
            result.powergrid = fit_data.validate_powergrid_verbose(uad, calc, fit);
        }
        if options.calibration {
            result.calibration = fit_data.validate_calibration_verbose(uad, calc, fit);
        }
        if options.dronebay_volume {
            result.dronebay_volume = fit_data.validate_dronebay_volume_verbose(uad, calc, fit);
        }
        if options.drone_bandwidth {
            result.drone_bandwidth = fit_data.validate_drone_bandwidth_verbose(uad, calc, fit);
        }
        if options.rig_slots {
            result.rig_slots = fit_data.validate_rig_slots_verbose(uad, calc, fit);
        }
        if options.subsystem_slots {
            result.subsystem_slots = fit_data.validate_subsystem_slots_verbose(uad, calc, fit);
        }
        if options.launched_drones {
            result.launched_drones = fit_data.validate_launched_drones_verbose(uad, calc, fit);
        }
        if options.launched_fighters {
            result.launched_fighters = fit_data.validate_launched_fighters_verbose(uad, calc, fit);
        }
        if options.launched_support_fighters {
            result.launched_support_fighters = fit_data.validate_launched_support_fighters_verbose(uad, calc, fit);
        }
        if options.launched_light_fighters {
            result.launched_light_fighters = fit_data.validate_launched_light_fighters_verbose(uad, calc, fit);
        }
        if options.launched_heavy_fighters {
            result.launched_heavy_fighters = fit_data.validate_launched_heavy_fighters_verbose(uad, calc, fit);
        }
        if options.launched_standup_support_fighters {
            result.launched_standup_support_fighters =
                fit_data.validate_launched_standup_support_fighters_verbose(uad, calc, fit);
        }
        if options.launched_standup_light_fighters {
            result.launched_standup_light_fighters =
                fit_data.validate_launched_standup_light_fighters_verbose(uad, calc, fit);
        }
        if options.launched_standup_heavy_fighters {
            result.launched_standup_heavy_fighters =
                fit_data.validate_launched_standup_heavy_fighters_verbose(uad, calc, fit);
        }
        if options.turret_slots {
            result.turret_slots = fit_data.validate_turret_slots_verbose(uad, calc, fit);
        }
        if options.launcher_slots {
            result.launcher_slots = fit_data.validate_launcher_slots_verbose(uad, calc, fit);
        }
        if options.high_slots {
            result.high_slots = fit_data.validate_high_slots_verbose(uad, calc, fit);
        }
        if options.mid_slots {
            result.mid_slots = fit_data.validate_mid_slots_verbose(uad, calc, fit);
        }
        if options.low_slots {
            result.low_slots = fit_data.validate_low_slots_verbose(uad, calc, fit);
        }
        if options.implant_slot_index {
            result.implant_slot_index = fit_data.validate_implant_slot_index_verbose();
        }
        if options.booster_slot_index {
            result.booster_slot_index = fit_data.validate_booster_slot_index_verbose();
        }
        if options.subsystem_slot_index {
            result.subsystem_slot_index = fit_data.validate_subsystem_slot_index_verbose();
        }
        if options.ship_limit {
            result.ship_limit = fit_data.validate_ship_limit_verbose(uad, fit);
        }
        if options.max_group_fitted {
            result.max_group_fitted = fit_data.validate_max_group_fitted_verbose(uad, calc);
        }
        if options.max_group_online {
            result.max_group_online = fit_data.validate_max_group_online_verbose(uad, calc);
        }
        if options.max_group_active {
            result.max_group_active = fit_data.validate_max_group_active_verbose(uad, calc);
        }
        if options.rig_size {
            result.rig_size = fit_data.validate_rig_size_verbose(uad, fit);
        }
        if options.skill_reqs {
            result.skill_reqs = fit_data.validate_skill_reqs_verbose();
        }
        if options.charge_group {
            result.charge_group = fit_data.validate_charge_group_verbose(uad);
        }
        if options.charge_size {
            result.charge_size = fit_data.validate_charge_size_verbose(uad);
        }
        result
    }
}
