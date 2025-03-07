use crate::{
    info::{HValidInfo, HValidInfoMode},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HValidateFitCmd {
    default: bool,
    #[serde(default)]
    cpu: Option<HValidationOption>,
    #[serde(default)]
    powergrid: Option<HValidationOption>,
    #[serde(default)]
    calibration: Option<HValidationOption>,
    #[serde(default)]
    drone_bay_volume: Option<HValidationOption>,
    #[serde(default)]
    drone_bandwidth: Option<HValidationOption>,
    #[serde(default)]
    fighter_bay_volume: Option<HValidationOption>,
    #[serde(default)]
    rig_slot_count: Option<HValidationOption>,
    #[serde(default)]
    subsystem_slot_count: Option<HValidationOption>,
    #[serde(default)]
    launched_drone_count: Option<HValidationOption>,
    #[serde(default)]
    launched_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_support_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_light_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_heavy_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_standup_support_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_standup_light_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    launched_standup_heavy_fighter_count: Option<HValidationOption>,
    #[serde(default)]
    turret_slot_count: Option<HValidationOption>,
    #[serde(default)]
    launcher_slot_count: Option<HValidationOption>,
    #[serde(default)]
    high_slot_count: Option<HValidationOption>,
    #[serde(default)]
    mid_slot_count: Option<HValidationOption>,
    #[serde(default)]
    low_slot_count: Option<HValidationOption>,
    #[serde(default)]
    implant_slot_index: Option<HValidationOption>,
    #[serde(default)]
    booster_slot_index: Option<HValidationOption>,
    #[serde(default)]
    subsystem_slot_index: Option<HValidationOption>,
    #[serde(default)]
    ship_limit: Option<HValidationOption>,
    #[serde(default)]
    max_group_fitted: Option<HValidationOption>,
    #[serde(default)]
    max_group_online: Option<HValidationOption>,
    #[serde(default)]
    max_group_active: Option<HValidationOption>,
    #[serde(default)]
    rig_size: Option<HValidationOption>,
    #[serde(default)]
    skill_reqs: Option<HValidationOption>,
    #[serde(default)]
    charge_group: Option<HValidationOption>,
    #[serde(default)]
    charge_size: Option<HValidationOption>,
    #[serde(default)]
    charge_volume: Option<HValidationOption>,
    #[serde(default)]
    capital_module: Option<HValidationOption>,
    #[serde(default)]
    not_loaded_item: Option<HValidationOption>,
    #[serde(default)]
    module_state: Option<HValidationOption>,
    #[serde(default)]
    item_kind: Option<HValidationOption>,
    #[serde(default)]
    drone_group: Option<HValidationOption>,
    #[serde(default)]
    fighter_count: Option<HValidationOption>,
}
impl HValidateFitCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
        valid_mode: HValidInfoMode,
    ) -> Result<HValidInfo, HExecError> {
        let mut core_options = match self.default {
            true => rc::SolValOptions::all_enabled(),
            false => rc::SolValOptions::all_disabled(),
        };
        process_option(&self.cpu, &mut core_options.cpu);
        process_option(&self.powergrid, &mut core_options.powergrid);
        process_option(&self.calibration, &mut core_options.calibration);
        process_option(&self.drone_bay_volume, &mut core_options.drone_bay_volume);
        process_option(&self.drone_bandwidth, &mut core_options.drone_bandwidth);
        process_option(&self.fighter_bay_volume, &mut core_options.fighter_bay_volume);
        process_option(&self.rig_slot_count, &mut core_options.rig_slot_count);
        process_option(&self.subsystem_slot_count, &mut core_options.subsystem_slot_count);
        process_option(&self.launched_drone_count, &mut core_options.launched_drone_count);
        process_option(&self.launched_fighter_count, &mut core_options.launched_fighter_count);
        process_option(
            &self.launched_support_fighter_count,
            &mut core_options.launched_support_fighter_count,
        );
        process_option(
            &self.launched_light_fighter_count,
            &mut core_options.launched_light_fighter_count,
        );
        process_option(
            &self.launched_heavy_fighter_count,
            &mut core_options.launched_heavy_fighter_count,
        );
        process_option(
            &self.launched_standup_support_fighter_count,
            &mut core_options.launched_standup_support_fighter_count,
        );
        process_option(
            &self.launched_standup_light_fighter_count,
            &mut core_options.launched_standup_light_fighter_count,
        );
        process_option(
            &self.launched_standup_heavy_fighter_count,
            &mut core_options.launched_standup_heavy_fighter_count,
        );
        process_option(&self.turret_slot_count, &mut core_options.turret_slot_count);
        process_option(&self.launcher_slot_count, &mut core_options.launcher_slot_count);
        process_option(&self.high_slot_count, &mut core_options.high_slot_count);
        process_option(&self.mid_slot_count, &mut core_options.mid_slot_count);
        process_option(&self.low_slot_count, &mut core_options.low_slot_count);
        process_option(&self.implant_slot_index, &mut core_options.implant_slot_index);
        process_option(&self.booster_slot_index, &mut core_options.booster_slot_index);
        process_option(&self.subsystem_slot_index, &mut core_options.subsystem_slot_index);
        process_option(&self.ship_limit, &mut core_options.ship_limit);
        process_option(&self.max_group_fitted, &mut core_options.max_group_fitted);
        process_option(&self.max_group_online, &mut core_options.max_group_online);
        process_option(&self.max_group_active, &mut core_options.max_group_active);
        process_option(&self.rig_size, &mut core_options.rig_size);
        process_option(&self.skill_reqs, &mut core_options.skill_reqs);
        process_option(&self.charge_group, &mut core_options.charge_group);
        process_option(&self.charge_size, &mut core_options.charge_size);
        process_option(&self.charge_volume, &mut core_options.charge_volume);
        process_option(&self.capital_module, &mut core_options.capital_module);
        process_option(&self.not_loaded_item, &mut core_options.not_loaded_item);
        process_option(&self.module_state, &mut core_options.module_state);
        process_option(&self.item_kind, &mut core_options.item_kind);
        process_option(&self.drone_group, &mut core_options.drone_group);
        process_option(&self.fighter_count, &mut core_options.fighter_count);
        // Run validation
        match valid_mode {
            HValidInfoMode::Simple => core_sol.validate_fit_fast(fit_id, &core_options).map(|v| v.into()),
            HValidInfoMode::Detailed => core_sol
                .validate_fit_verbose(fit_id, &core_options)
                .map(|v| (&v).into()),
        }
        .map_err(|core_error| match core_error {
            rc::err::ValidateFitError::FitNotFound(e) => HExecError::FitNotFoundPrimary(e),
        })
    }
}
impl Default for HValidateFitCmd {
    fn default() -> Self {
        Self {
            default: true,
            cpu: None,
            powergrid: None,
            calibration: None,
            drone_bay_volume: None,
            drone_bandwidth: None,
            fighter_bay_volume: None,
            rig_slot_count: None,
            subsystem_slot_count: None,
            launched_drone_count: None,
            launched_fighter_count: None,
            launched_support_fighter_count: None,
            launched_light_fighter_count: None,
            launched_heavy_fighter_count: None,
            launched_standup_support_fighter_count: None,
            launched_standup_light_fighter_count: None,
            launched_standup_heavy_fighter_count: None,
            turret_slot_count: None,
            launcher_slot_count: None,
            high_slot_count: None,
            mid_slot_count: None,
            low_slot_count: None,
            implant_slot_index: None,
            booster_slot_index: None,
            subsystem_slot_index: None,
            ship_limit: None,
            max_group_fitted: None,
            max_group_online: None,
            max_group_active: None,
            rig_size: None,
            skill_reqs: None,
            charge_group: None,
            charge_size: None,
            charge_volume: None,
            capital_module: None,
            not_loaded_item: None,
            module_state: None,
            item_kind: None,
            drone_group: None,
            fighter_count: None,
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum HValidationOption {
    Simple(bool),
    Extended(
        bool,
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")] Vec<rc::SolItemId>,
    ),
}
impl HValidationOption {
    fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    fn get_ignored_item_ids(&self) -> Vec<rc::SolItemId> {
        match self {
            Self::Simple(_) => Vec::new(),
            Self::Extended(_, ignored_item_ids) => ignored_item_ids.clone(),
        }
    }
}

fn process_option(option: &Option<HValidationOption>, core_option: &mut rc::SolValOption) {
    if let Some(option) = option {
        core_option.enabled = option.is_enabled();
        core_option.ignored_item_ids = option.get_ignored_item_ids();
    }
}
