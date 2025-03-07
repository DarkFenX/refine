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
        options: SolValOptions,
    ) -> bool {
        let ship = fit.ship.map(|v| uad.items.get_item(&v).unwrap().get_ship().unwrap());
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data_mut(&fit.id).unwrap();
        // Order of validations matters here; the faster validation and the more likely it is to
        // fail, the closer to top it should be
        if options.cpu && !fit_data.validate_cpu_fast(uad, calc, fit) {
            return false;
        }
        if options.powergrid && !fit_data.validate_powergrid_fast(uad, calc, fit) {
            return false;
        }
        if options.calibration && !fit_data.validate_calibration_fast(uad, calc, fit) {
            return false;
        }
        if options.drone_bay_volume && !fit_data.validate_drone_bay_volume_fast(uad, calc, fit) {
            return false;
        }
        if options.drone_bandwidth && !fit_data.validate_drone_bandwidth_fast(uad, calc, fit) {
            return false;
        }
        if options.fighter_bay_volume && !fit_data.validate_fighter_bay_volume_fast(uad, calc, fit) {
            return false;
        }
        if options.rig_slot_count && !fit_data.validate_rig_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.subsystem_slot_count && !fit_data.validate_subsystem_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.launched_drone_count && !fit_data.validate_launched_drone_count_fast(uad, calc, fit) {
            return false;
        }
        if options.launched_fighter_count && !fit_data.validate_launched_fighter_count_fast(uad, calc, fit) {
            return false;
        }
        if options.launched_support_fighter_count
            && !fit_data.validate_launched_support_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.launched_light_fighter_count && !fit_data.validate_launched_light_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.launched_heavy_fighter_count && !fit_data.validate_launched_heavy_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.launched_standup_support_fighter_count
            && !fit_data.validate_launched_standup_support_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.launched_standup_light_fighter_count
            && !fit_data.validate_launched_standup_light_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.launched_standup_heavy_fighter_count
            && !fit_data.validate_launched_standup_heavy_fighter_count_fast(uad, calc, fit)
        {
            return false;
        }
        if options.turret_slot_count && !fit_data.validate_turret_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.launcher_slot_count && !fit_data.validate_launcher_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.high_slot_count && !fit_data.validate_high_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.mid_slot_count && !fit_data.validate_mid_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.low_slot_count && !fit_data.validate_low_slot_count_fast(uad, calc, fit) {
            return false;
        }
        if options.implant_slot_index && !fit_data.validate_implant_slot_index_fast() {
            return false;
        }
        if options.booster_slot_index && !fit_data.validate_booster_slot_index_fast() {
            return false;
        }
        if options.subsystem_slot_index && !fit_data.validate_subsystem_slot_index_fast() {
            return false;
        }
        if options.ship_limit && !fit_data.validate_ship_limit_fast(ship) {
            return false;
        }
        if options.max_group_fitted && !fit_data.validate_max_group_fitted_fast(uad, calc) {
            return false;
        }
        if options.max_group_online && !fit_data.validate_max_group_online_fast(uad, calc) {
            return false;
        }
        if options.max_group_active && !fit_data.validate_max_group_active_fast(uad, calc) {
            return false;
        }
        if options.rig_size && !fit_data.validate_rig_size_fast(ship) {
            return false;
        }
        if options.skill_reqs && !fit_data.validate_skill_reqs_fast() {
            return false;
        }
        if options.charge_group && !fit_data.validate_charge_group_fast(uad) {
            return false;
        }
        if options.charge_size && !fit_data.validate_charge_size_fast(uad) {
            return false;
        }
        if options.charge_volume && !fit_data.validate_charge_volume_fast(uad) {
            return false;
        }
        if options.capital_module && !fit_data.validate_capital_module_fast(ship) {
            return false;
        }
        if options.not_loaded_item && !fit_data.validate_not_loaded_item_fast() {
            return false;
        }
        if options.module_state && !fit_data.validate_module_state_fast() {
            return false;
        }
        if options.item_kind && !fit_data.validate_item_kind_fast() {
            return false;
        }
        if options.drone_group && !fit_data.validate_drone_group_fast() {
            return false;
        }
        if options.fighter_count && !fit_data.validate_fighter_count_fast() {
            return false;
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
        let ship = fit.ship.map(|v| uad.items.get_item(&v).unwrap().get_ship().unwrap());
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
        if options.drone_bay_volume {
            result.drone_bay_volume = fit_data.validate_drone_bay_volume_verbose(uad, calc, fit);
        }
        if options.drone_bandwidth {
            result.drone_bandwidth = fit_data.validate_drone_bandwidth_verbose(uad, calc, fit);
        }
        if options.fighter_bay_volume {
            result.fighter_bay_volume = fit_data.validate_fighter_bay_volume_verbose(uad, calc, fit);
        }
        if options.rig_slot_count {
            result.rig_slot_count = fit_data.validate_rig_slot_count_verbose(uad, calc, fit);
        }
        if options.subsystem_slot_count {
            result.subsystem_slot_count = fit_data.validate_subsystem_slot_count_verbose(uad, calc, fit);
        }
        if options.launched_drone_count {
            result.launched_drone_count = fit_data.validate_launched_drone_count_verbose(uad, calc, fit);
        }
        if options.launched_fighter_count {
            result.launched_fighter_count = fit_data.validate_launched_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_support_fighter_count {
            result.launched_support_fighter_count =
                fit_data.validate_launched_support_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_light_fighter_count {
            result.launched_light_fighter_count =
                fit_data.validate_launched_light_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_heavy_fighter_count {
            result.launched_heavy_fighter_count =
                fit_data.validate_launched_heavy_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_standup_support_fighter_count {
            result.launched_standup_support_fighter_count =
                fit_data.validate_launched_standup_support_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_standup_light_fighter_count {
            result.launched_standup_light_fighter_count =
                fit_data.validate_launched_standup_light_fighter_count_verbose(uad, calc, fit);
        }
        if options.launched_standup_heavy_fighter_count {
            result.launched_standup_heavy_fighter_count =
                fit_data.validate_launched_standup_heavy_fighter_count_verbose(uad, calc, fit);
        }
        if options.turret_slot_count {
            result.turret_slot_count = fit_data.validate_turret_slot_count_verbose(uad, calc, fit);
        }
        if options.launcher_slot_count {
            result.launcher_slot_count = fit_data.validate_launcher_slot_count_verbose(uad, calc, fit);
        }
        if options.high_slot_count {
            result.high_slot_count = fit_data.validate_high_slot_count_verbose(uad, calc, fit);
        }
        if options.mid_slot_count {
            result.mid_slot_count = fit_data.validate_mid_slot_count_verbose(uad, calc, fit);
        }
        if options.low_slot_count {
            result.low_slot_count = fit_data.validate_low_slot_count_verbose(uad, calc, fit);
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
            result.ship_limit = fit_data.validate_ship_limit_verbose(ship);
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
            result.rig_size = fit_data.validate_rig_size_verbose(ship);
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
        if options.charge_volume {
            result.charge_volume = fit_data.validate_charge_volume_verbose(uad);
        }
        if options.capital_module {
            result.capital_module = fit_data.validate_capital_module_verbose(ship);
        }
        if options.not_loaded_item {
            result.not_loaded_item = fit_data.validate_not_loaded_item_verbose();
        }
        if options.module_state {
            result.module_state = fit_data.validate_module_state_verbose();
        }
        if options.item_kind {
            result.item_kind = fit_data.validate_item_kind_verbose();
        }
        if options.drone_group {
            result.drone_group = fit_data.validate_drone_group_verbose();
        }
        if options.fighter_count {
            result.fighter_count = fit_data.validate_fighter_count_verbose();
        }
        result
    }
}
