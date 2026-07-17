use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::util::default_true;

#[derive(educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HValOptions {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    // Generic
    not_loaded_item: Option<HValOption>,
    item_kind: Option<HValOption>,
    skill_reqs: Option<HValOption>,
    // Implants/boosters
    implant_slot_index: Option<HValOption>,
    booster_slot_index: Option<HValOption>,
    // Shared between mod-alike items
    cpu: Option<HValOption>,
    powergrid: Option<HValOption>,
    ship_limit: Option<HValOption>,
    max_group_fitted: Option<HValOption>,
    max_group_online: Option<HValOption>,
    max_group_active: Option<HValOption>,
    max_type_fitted: Option<HValOption>,
    item_vs_ship_kind: Option<HValOption>,
    // Modules
    high_slot_count: Option<HValOption>,
    mid_slot_count: Option<HValOption>,
    low_slot_count: Option<HValOption>,
    turret_slot_count: Option<HValOption>,
    launcher_slot_count: Option<HValOption>,
    module_state: Option<HValOption>,
    capital_module: Option<HValOption>,
    overload_skill: Option<HValOption>,
    unusable_cap: Option<HValOption>,
    // Charges
    charge_group: Option<HValOption>,
    charge_parent_group: Option<HValOption>,
    charge_size: Option<HValOption>,
    charge_volume: Option<HValOption>,
    // Rigs
    rig_slot_count: Option<HValOption>,
    calibration: Option<HValOption>,
    rig_size: Option<HValOption>,
    // Services
    service_slot_count: Option<HValOption>,
    // T3 subsystems/stances
    subsystem_slot_count: Option<HValOption>,
    subsystem_slot_index: Option<HValOption>,
    ship_stance: Option<HValOption>,
    // Drones
    drone_bay_volume: Option<HValOption>,
    launched_drone_count: Option<HValOption>,
    drone_bandwidth: Option<HValOption>,
    unlaunchable_drone_slot: Option<HValOption>,
    unlaunchable_drone_bandwidth: Option<HValOption>,
    drone_group: Option<HValOption>,
    // Fighters
    fighter_bay_volume: Option<HValOption>,
    launched_fighter_count: Option<HValOption>,
    launched_light_fighter_count: Option<HValOption>,
    launched_heavy_fighter_count: Option<HValOption>,
    launched_support_fighter_count: Option<HValOption>,
    launched_st_light_fighter_count: Option<HValOption>,
    launched_st_heavy_fighter_count: Option<HValOption>,
    launched_st_support_fighter_count: Option<HValOption>,
    unlaunchable_fighter: Option<HValOption>,
    unlaunchable_light_fighter: Option<HValOption>,
    unlaunchable_heavy_fighter: Option<HValOption>,
    unlaunchable_support_fighter: Option<HValOption>,
    unlaunchable_st_light_fighter: Option<HValOption>,
    unlaunchable_st_heavy_fighter: Option<HValOption>,
    unlaunchable_st_support_fighter: Option<HValOption>,
    fighter_squad_size: Option<HValOption>,
    // Projection, destination side
    activation_blocked: Option<HValOption>,
    effect_stopper: Option<HValOption>,
    cloaking_blocked: Option<HValOption>,
    // Projection, source side
    projectee_filter: Option<HValOption>,
    assist_immunity: Option<HValOption>,
    offense_immunity: Option<HValOption>,
    resist_immunity: Option<HValOption>,
    // Sec zone
    sec_zone_fitted: Option<HValOption>,
    sec_zone_online: Option<HValOption>,
    sec_zone_active: Option<HValOption>,
    sec_zone_unonlineable: Option<HValOption>,
    sec_zone_unactivable: Option<HValOption>,
    sec_zone_effect: Option<HValOption>,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
enum HValOption {
    Simple(bool),
    Extended(bool, #[serde_as(as = "Vec<DisplayFromStr>")] Vec<rc::ItemId>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValOptions {
    pub(in crate::cmd) fn to_core(&self) -> rc::val::ValOptions {
        let mut core_options = match self.default {
            true => rc::val::ValOptions::all_enabled(),
            false => rc::val::ValOptions::all_disabled(),
        };
        // Generic
        process_option(&self.not_loaded_item, &mut core_options.not_loaded_item);
        process_option(&self.item_kind, &mut core_options.item_kind);
        process_option(&self.skill_reqs, &mut core_options.skill_reqs);
        // Implants/boosters
        process_option(&self.implant_slot_index, &mut core_options.implant_slot_index);
        process_option(&self.booster_slot_index, &mut core_options.booster_slot_index);
        // Shared between mod-alike items
        process_option(&self.cpu, &mut core_options.cpu);
        process_option(&self.powergrid, &mut core_options.powergrid);
        process_option(&self.ship_limit, &mut core_options.ship_limit);
        process_option(&self.max_group_fitted, &mut core_options.max_group_fitted);
        process_option(&self.max_group_online, &mut core_options.max_group_online);
        process_option(&self.max_group_active, &mut core_options.max_group_active);
        process_option(&self.max_type_fitted, &mut core_options.max_type_fitted);
        process_option(&self.item_vs_ship_kind, &mut core_options.item_vs_ship_kind);
        // Modules
        process_option(&self.high_slot_count, &mut core_options.high_slot_count);
        process_option(&self.mid_slot_count, &mut core_options.mid_slot_count);
        process_option(&self.low_slot_count, &mut core_options.low_slot_count);
        process_option(&self.turret_slot_count, &mut core_options.turret_slot_count);
        process_option(&self.launcher_slot_count, &mut core_options.launcher_slot_count);
        process_option(&self.module_state, &mut core_options.module_state);
        process_option(&self.capital_module, &mut core_options.capital_module);
        process_option(&self.overload_skill, &mut core_options.overload_skill);
        process_option(&self.unusable_cap, &mut core_options.unusable_cap);
        // Charges
        process_option(&self.charge_group, &mut core_options.charge_group);
        process_option(&self.charge_parent_group, &mut core_options.charge_parent_group);
        process_option(&self.charge_size, &mut core_options.charge_size);
        process_option(&self.charge_volume, &mut core_options.charge_volume);
        // Rigs
        process_option(&self.rig_slot_count, &mut core_options.rig_slot_count);
        process_option(&self.calibration, &mut core_options.calibration);
        process_option(&self.rig_size, &mut core_options.rig_size);
        // Services
        process_option(&self.service_slot_count, &mut core_options.service_slot_count);
        // T3 subsystems/stances
        process_option(&self.subsystem_slot_count, &mut core_options.subsystem_slot_count);
        process_option(&self.subsystem_slot_index, &mut core_options.subsystem_slot_index);
        process_option(&self.ship_stance, &mut core_options.ship_stance);
        // Drones
        process_option(&self.drone_bay_volume, &mut core_options.drone_bay_volume);
        process_option(&self.launched_drone_count, &mut core_options.launched_drone_count);
        process_option(&self.drone_bandwidth, &mut core_options.drone_bandwidth);
        process_option(&self.unlaunchable_drone_slot, &mut core_options.unlaunchable_drone_slot);
        process_option(
            &self.unlaunchable_drone_bandwidth,
            &mut core_options.unlaunchable_drone_bandwidth,
        );
        process_option(&self.drone_group, &mut core_options.drone_group);
        // Fighters
        process_option(&self.fighter_bay_volume, &mut core_options.fighter_bay_volume);
        process_option(&self.launched_fighter_count, &mut core_options.launched_fighter_count);
        process_option(
            &self.launched_light_fighter_count,
            &mut core_options.launched_light_fighter_count,
        );
        process_option(
            &self.launched_heavy_fighter_count,
            &mut core_options.launched_heavy_fighter_count,
        );
        process_option(
            &self.launched_support_fighter_count,
            &mut core_options.launched_support_fighter_count,
        );
        process_option(
            &self.launched_st_light_fighter_count,
            &mut core_options.launched_st_light_fighter_count,
        );
        process_option(
            &self.launched_st_heavy_fighter_count,
            &mut core_options.launched_st_heavy_fighter_count,
        );
        process_option(
            &self.launched_st_support_fighter_count,
            &mut core_options.launched_st_support_fighter_count,
        );
        process_option(&self.unlaunchable_fighter, &mut core_options.unlaunchable_fighter);
        process_option(
            &self.unlaunchable_light_fighter,
            &mut core_options.unlaunchable_light_fighter,
        );
        process_option(
            &self.unlaunchable_heavy_fighter,
            &mut core_options.unlaunchable_heavy_fighter,
        );
        process_option(
            &self.unlaunchable_support_fighter,
            &mut core_options.unlaunchable_support_fighter,
        );
        process_option(
            &self.unlaunchable_st_light_fighter,
            &mut core_options.unlaunchable_st_light_fighter,
        );
        process_option(
            &self.unlaunchable_st_heavy_fighter,
            &mut core_options.unlaunchable_st_heavy_fighter,
        );
        process_option(
            &self.unlaunchable_st_support_fighter,
            &mut core_options.unlaunchable_st_support_fighter,
        );
        process_option(&self.fighter_squad_size, &mut core_options.fighter_squad_size);
        // Projection, destination side
        process_option(&self.activation_blocked, &mut core_options.activation_blocked);
        process_option(&self.effect_stopper, &mut core_options.effect_stopper);
        process_option(&self.cloaking_blocked, &mut core_options.cloaking_blocked);
        // Projection, source side
        process_option(&self.projectee_filter, &mut core_options.projectee_filter);
        process_option(&self.assist_immunity, &mut core_options.assist_immunity);
        process_option(&self.offense_immunity, &mut core_options.offense_immunity);
        process_option(&self.resist_immunity, &mut core_options.resist_immunity);
        // Sec zone
        process_option(&self.sec_zone_fitted, &mut core_options.sec_zone_fitted);
        process_option(&self.sec_zone_online, &mut core_options.sec_zone_online);
        process_option(&self.sec_zone_active, &mut core_options.sec_zone_active);
        process_option(&self.sec_zone_unonlineable, &mut core_options.sec_zone_unonlineable);
        process_option(&self.sec_zone_unactivable, &mut core_options.sec_zone_unactivable);
        process_option(&self.sec_zone_effect, &mut core_options.sec_zone_effect);
        core_options
    }
}

fn process_option(option: &Option<HValOption>, core_option: &mut rc::val::ValOption) {
    if let Some(option) = option {
        *core_option = option.to_core();
    }
}

impl HValOption {
    fn to_core(&self) -> rc::val::ValOption {
        match self {
            Self::Simple(enabled) => match enabled {
                true => rc::val::ValOption::new_enabled(),
                false => rc::val::ValOption::new_disabled(),
            },
            Self::Extended(enabled, known_failures) => match enabled {
                true => rc::val::ValOption::Enabled(rc::val::ValEnabled {
                    kfs: known_failures.clone(),
                }),
                false => rc::val::ValOption::new_disabled(),
            },
        }
    }
}
