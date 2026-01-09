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
    #[serde(default)]
    not_loaded_item: Option<HValOption>,
    #[serde(default)]
    item_kind: Option<HValOption>,
    #[serde(default)]
    skill_reqs: Option<HValOption>,
    // Implants/boosters
    #[serde(default)]
    implant_slot_index: Option<HValOption>,
    #[serde(default)]
    booster_slot_index: Option<HValOption>,
    // Shared between mod-alike items
    #[serde(default)]
    cpu: Option<HValOption>,
    #[serde(default)]
    powergrid: Option<HValOption>,
    #[serde(default)]
    ship_limit: Option<HValOption>,
    #[serde(default)]
    max_group_fitted: Option<HValOption>,
    #[serde(default)]
    max_group_online: Option<HValOption>,
    #[serde(default)]
    max_group_active: Option<HValOption>,
    #[serde(default)]
    max_type_fitted: Option<HValOption>,
    #[serde(default)]
    item_vs_ship_kind: Option<HValOption>,
    // Modules
    #[serde(default)]
    high_slot_count: Option<HValOption>,
    #[serde(default)]
    mid_slot_count: Option<HValOption>,
    #[serde(default)]
    low_slot_count: Option<HValOption>,
    #[serde(default)]
    turret_slot_count: Option<HValOption>,
    #[serde(default)]
    launcher_slot_count: Option<HValOption>,
    #[serde(default)]
    module_state: Option<HValOption>,
    #[serde(default)]
    capital_module: Option<HValOption>,
    #[serde(default)]
    overload_skill: Option<HValOption>,
    #[serde(default)]
    unusable_cap: Option<HValOption>,
    // Charges
    #[serde(default)]
    charge_group: Option<HValOption>,
    #[serde(default)]
    charge_parent_group: Option<HValOption>,
    #[serde(default)]
    charge_size: Option<HValOption>,
    #[serde(default)]
    charge_volume: Option<HValOption>,
    // Rigs
    #[serde(default)]
    rig_slot_count: Option<HValOption>,
    #[serde(default)]
    calibration: Option<HValOption>,
    #[serde(default)]
    rig_size: Option<HValOption>,
    // Services
    #[serde(default)]
    service_slot_count: Option<HValOption>,
    // T3 subsystems/stances
    #[serde(default)]
    subsystem_slot_count: Option<HValOption>,
    #[serde(default)]
    subsystem_slot_index: Option<HValOption>,
    #[serde(default)]
    ship_stance: Option<HValOption>,
    // Drones
    #[serde(default)]
    drone_bay_volume: Option<HValOption>,
    #[serde(default)]
    launched_drone_count: Option<HValOption>,
    #[serde(default)]
    drone_bandwidth: Option<HValOption>,
    #[serde(default)]
    unlaunchable_drone_slot: Option<HValOption>,
    #[serde(default)]
    unlaunchable_drone_bandwidth: Option<HValOption>,
    #[serde(default)]
    drone_group: Option<HValOption>,
    // Fighters
    #[serde(default)]
    fighter_bay_volume: Option<HValOption>,
    #[serde(default)]
    launched_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_light_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_heavy_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_support_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_st_light_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_st_heavy_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_st_support_fighter_count: Option<HValOption>,
    #[serde(default)]
    unlaunchable_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_light_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_heavy_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_support_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_st_light_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_st_heavy_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_st_support_fighter: Option<HValOption>,
    #[serde(default)]
    fighter_squad_size: Option<HValOption>,
    // Projection, destination side
    #[serde(default)]
    activation_blocked: Option<HValOption>,
    #[serde(default)]
    effect_stopper: Option<HValOption>,
    // Projection, source side
    #[serde(default)]
    projectee_filter: Option<HValOption>,
    #[serde(default)]
    assist_immunity: Option<HValOption>,
    #[serde(default)]
    offense_immunity: Option<HValOption>,
    #[serde(default)]
    resist_immunity: Option<HValOption>,
    // Sec zone
    #[serde(default)]
    sec_zone_fitted: Option<HValOption>,
    #[serde(default)]
    sec_zone_online: Option<HValOption>,
    #[serde(default)]
    sec_zone_active: Option<HValOption>,
    #[serde(default)]
    sec_zone_unonlineable: Option<HValOption>,
    #[serde(default)]
    sec_zone_unactivable: Option<HValOption>,
    #[serde(default)]
    sec_zone_effect: Option<HValOption>,
}
impl From<&HValOptions> for rc::val::ValOptions {
    fn from(h_options: &HValOptions) -> Self {
        let mut core_options = match h_options.default {
            true => rc::val::ValOptions::all_enabled(),
            false => rc::val::ValOptions::all_disabled(),
        };
        // Generic
        process_option(&h_options.not_loaded_item, &mut core_options.not_loaded_item);
        process_option(&h_options.item_kind, &mut core_options.item_kind);
        process_option(&h_options.skill_reqs, &mut core_options.skill_reqs);
        // Implants/boosters
        process_option(&h_options.implant_slot_index, &mut core_options.implant_slot_index);
        process_option(&h_options.booster_slot_index, &mut core_options.booster_slot_index);
        // Shared between mod-alike items
        process_option(&h_options.cpu, &mut core_options.cpu);
        process_option(&h_options.powergrid, &mut core_options.powergrid);
        process_option(&h_options.ship_limit, &mut core_options.ship_limit);
        process_option(&h_options.max_group_fitted, &mut core_options.max_group_fitted);
        process_option(&h_options.max_group_online, &mut core_options.max_group_online);
        process_option(&h_options.max_group_active, &mut core_options.max_group_active);
        process_option(&h_options.max_type_fitted, &mut core_options.max_type_fitted);
        process_option(&h_options.item_vs_ship_kind, &mut core_options.item_vs_ship_kind);
        // Modules
        process_option(&h_options.high_slot_count, &mut core_options.high_slot_count);
        process_option(&h_options.mid_slot_count, &mut core_options.mid_slot_count);
        process_option(&h_options.low_slot_count, &mut core_options.low_slot_count);
        process_option(&h_options.turret_slot_count, &mut core_options.turret_slot_count);
        process_option(&h_options.launcher_slot_count, &mut core_options.launcher_slot_count);
        process_option(&h_options.module_state, &mut core_options.module_state);
        process_option(&h_options.capital_module, &mut core_options.capital_module);
        process_option(&h_options.overload_skill, &mut core_options.overload_skill);
        process_option(&h_options.unusable_cap, &mut core_options.unusable_cap);
        // Charges
        process_option(&h_options.charge_group, &mut core_options.charge_group);
        process_option(&h_options.charge_parent_group, &mut core_options.charge_parent_group);
        process_option(&h_options.charge_size, &mut core_options.charge_size);
        process_option(&h_options.charge_volume, &mut core_options.charge_volume);
        // Rigs
        process_option(&h_options.rig_slot_count, &mut core_options.rig_slot_count);
        process_option(&h_options.calibration, &mut core_options.calibration);
        process_option(&h_options.rig_size, &mut core_options.rig_size);
        // Services
        process_option(&h_options.service_slot_count, &mut core_options.service_slot_count);
        // T3 subsystems/stances
        process_option(&h_options.subsystem_slot_count, &mut core_options.subsystem_slot_count);
        process_option(&h_options.subsystem_slot_index, &mut core_options.subsystem_slot_index);
        process_option(&h_options.ship_stance, &mut core_options.ship_stance);
        // Drones
        process_option(&h_options.drone_bay_volume, &mut core_options.drone_bay_volume);
        process_option(&h_options.launched_drone_count, &mut core_options.launched_drone_count);
        process_option(&h_options.drone_bandwidth, &mut core_options.drone_bandwidth);
        process_option(
            &h_options.unlaunchable_drone_slot,
            &mut core_options.unlaunchable_drone_slot,
        );
        process_option(
            &h_options.unlaunchable_drone_bandwidth,
            &mut core_options.unlaunchable_drone_bandwidth,
        );
        process_option(&h_options.drone_group, &mut core_options.drone_group);
        // Fighters
        process_option(&h_options.fighter_bay_volume, &mut core_options.fighter_bay_volume);
        process_option(
            &h_options.launched_fighter_count,
            &mut core_options.launched_fighter_count,
        );
        process_option(
            &h_options.launched_light_fighter_count,
            &mut core_options.launched_light_fighter_count,
        );
        process_option(
            &h_options.launched_heavy_fighter_count,
            &mut core_options.launched_heavy_fighter_count,
        );
        process_option(
            &h_options.launched_support_fighter_count,
            &mut core_options.launched_support_fighter_count,
        );
        process_option(
            &h_options.launched_st_light_fighter_count,
            &mut core_options.launched_st_light_fighter_count,
        );
        process_option(
            &h_options.launched_st_heavy_fighter_count,
            &mut core_options.launched_st_heavy_fighter_count,
        );
        process_option(
            &h_options.launched_st_support_fighter_count,
            &mut core_options.launched_st_support_fighter_count,
        );
        process_option(&h_options.unlaunchable_fighter, &mut core_options.unlaunchable_fighter);
        process_option(
            &h_options.unlaunchable_light_fighter,
            &mut core_options.unlaunchable_light_fighter,
        );
        process_option(
            &h_options.unlaunchable_heavy_fighter,
            &mut core_options.unlaunchable_heavy_fighter,
        );
        process_option(
            &h_options.unlaunchable_support_fighter,
            &mut core_options.unlaunchable_support_fighter,
        );
        process_option(
            &h_options.unlaunchable_st_light_fighter,
            &mut core_options.unlaunchable_st_light_fighter,
        );
        process_option(
            &h_options.unlaunchable_st_heavy_fighter,
            &mut core_options.unlaunchable_st_heavy_fighter,
        );
        process_option(
            &h_options.unlaunchable_st_support_fighter,
            &mut core_options.unlaunchable_st_support_fighter,
        );
        process_option(&h_options.fighter_squad_size, &mut core_options.fighter_squad_size);
        // Projection, destination side
        process_option(&h_options.activation_blocked, &mut core_options.activation_blocked);
        process_option(&h_options.effect_stopper, &mut core_options.effect_stopper);
        // Projection, source side
        process_option(&h_options.projectee_filter, &mut core_options.projectee_filter);
        process_option(&h_options.assist_immunity, &mut core_options.assist_immunity);
        process_option(&h_options.offense_immunity, &mut core_options.offense_immunity);
        process_option(&h_options.resist_immunity, &mut core_options.resist_immunity);
        // Sec zone
        process_option(&h_options.sec_zone_fitted, &mut core_options.sec_zone_fitted);
        process_option(&h_options.sec_zone_online, &mut core_options.sec_zone_online);
        process_option(&h_options.sec_zone_active, &mut core_options.sec_zone_active);
        process_option(
            &h_options.sec_zone_unonlineable,
            &mut core_options.sec_zone_unonlineable,
        );
        process_option(&h_options.sec_zone_unactivable, &mut core_options.sec_zone_unactivable);
        process_option(&h_options.sec_zone_effect, &mut core_options.sec_zone_effect);
        core_options
    }
}

#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
enum HValOption {
    Simple(bool),
    Extended(bool, #[serde_as(as = "Vec<DisplayFromStr>")] Vec<rc::ItemId>),
}
impl HValOption {
    fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    fn get_known_failures(&self) -> Vec<rc::ItemId> {
        match self {
            Self::Simple(_) => Vec::new(),
            Self::Extended(_, known_failures) => known_failures.clone(),
        }
    }
}

fn process_option(option: &Option<HValOption>, core_option: &mut rc::val::ValOption) {
    if let Some(option) = option {
        core_option.enabled = option.is_enabled();
        core_option.kfs = option.get_known_failures();
    }
}
