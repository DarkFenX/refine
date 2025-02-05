use crate::{
    info::{HValidInfo, HValidInfoMode},
    util::HExecError,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HValidFitCmd {
    Include(Vec<String>),
    Exclude(Vec<String>),
}
impl HValidFitCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
        valid_mode: HValidInfoMode,
    ) -> Result<HValidInfo, HExecError> {
        // Extract options
        let (mut core_options, options, alt_value) = match self {
            Self::Include(options) => (rc::SolValOptions::new_all_disabled(), options, true),
            Self::Exclude(options) => (rc::SolValOptions::new_all_enabled(), options, false),
        };
        for option in options {
            match option.as_str() {
                "cpu" => core_options.cpu = alt_value,
                "powergrid" => core_options.powergrid = alt_value,
                "calibration" => core_options.calibration = alt_value,
                "dronebay_volume" => core_options.dronebay_volume = alt_value,
                "drone_bandwidth" => core_options.drone_bandwidth = alt_value,
                "rig_slots" => core_options.rig_slots = alt_value,
                "subsystem_slots" => core_options.subsystem_slots = alt_value,
                "launched_drones" => core_options.launched_drones = alt_value,
                "launched_fighters" => core_options.launched_fighters = alt_value,
                "launched_support_fighters" => core_options.launched_support_fighters = alt_value,
                "launched_light_fighters" => core_options.launched_light_fighters = alt_value,
                "launched_heavy_fighters" => core_options.launched_heavy_fighters = alt_value,
                "launched_standup_support_fighters" => core_options.launched_standup_support_fighters = alt_value,
                "launched_standup_light_fighters" => core_options.launched_standup_light_fighters = alt_value,
                "launched_standup_heavy_fighters" => core_options.launched_standup_heavy_fighters = alt_value,
                "turret_slots" => core_options.turret_slots = alt_value,
                "launcher_slots" => core_options.launcher_slots = alt_value,
                "high_slots" => core_options.high_slots = alt_value,
                "mid_slots" => core_options.mid_slots = alt_value,
                "low_slots" => core_options.low_slots = alt_value,
                "implant_slot_index" => core_options.implant_slot_index = alt_value,
                "booster_slot_index" => core_options.booster_slot_index = alt_value,
                "subsystem_slot_index" => core_options.subsystem_slot_index = alt_value,
                "ship_limit" => core_options.ship_limit = alt_value,
                "max_group_fitted" => core_options.max_group_fitted = alt_value,
                "max_group_online" => core_options.max_group_online = alt_value,
                "max_group_active" => core_options.max_group_active = alt_value,
                "rig_size" => core_options.rig_size = alt_value,
                "skill_reqs" => core_options.skill_reqs = alt_value,
                "charge_group" => core_options.charge_group = alt_value,
                "charge_size" => core_options.charge_size = alt_value,
                "charge_volume" => core_options.charge_volume = alt_value,
                "capital_module" => core_options.capital_module = alt_value,
                "not_loaded_item" => core_options.not_loaded_item = alt_value,
                _ => (),
            }
        }
        // Run validation
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fit_fast(fit_id, core_options).map(|v| v.into()),
            HValidInfoMode::Detailed => core_sol.validate_fit_verbose(fit_id, core_options).map(|v| (&v).into()),
        }
        .map_err(|core_error| match core_error {
            rc::err::ValidateFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
        })
    }
}
impl Default for HValidFitCmd {
    fn default() -> Self {
        Self::Exclude(Vec::new())
    }
}
