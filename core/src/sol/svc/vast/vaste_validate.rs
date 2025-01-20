use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolValResult, SolVast},
    },
    uad::{fit::SolFit, SolUad},
};

impl SolVast {
    pub(in crate::sol) fn validate_fit_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> bool {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
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
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> SolValResult {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
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
        result
    }
}
