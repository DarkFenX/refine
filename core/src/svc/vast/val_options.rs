use itertools::Itertools;

use crate::{
    def::{FitId, ItemId},
    sol::SolarSystem,
    ud::{UFitKey, UItemKey},
    util::RSet,
};

/// Solar system validation options.
#[derive(Clone)]
pub struct ValOptionsSol {
    /// Fit IDs to validate.
    pub fit_ids: Vec<FitId>,
    /// Validation options.
    pub options: ValOptions,
}
impl ValOptionsSol {
    /// Initialize with all validations enabled.
    pub fn all_enabled() -> Self {
        Self {
            fit_ids: Vec::new(),
            options: ValOptions::all_enabled(),
        }
    }
    /// Initialize with all validations disabled.
    pub fn all_disabled() -> Self {
        Self {
            fit_ids: Vec::new(),
            options: ValOptions::all_disabled(),
        }
    }
}

/// Validation options.
#[derive(Clone)]
pub struct ValOptions {
    // Generic
    pub not_loaded_item: ValOption,
    pub item_kind: ValOption,
    pub skill_reqs: ValOption,
    // Implants/boosters
    pub implant_slot_index: ValOption,
    pub booster_slot_index: ValOption,
    // Shared between mod-alike items
    pub cpu: ValOption,
    pub powergrid: ValOption,
    pub ship_limit: ValOption,
    pub max_group_fitted: ValOption,
    pub max_group_online: ValOption,
    pub max_group_active: ValOption,
    pub max_type_fitted: ValOption,
    pub item_vs_ship_kind: ValOption,
    // Modules
    pub high_slot_count: ValOption,
    pub mid_slot_count: ValOption,
    pub low_slot_count: ValOption,
    pub turret_slot_count: ValOption,
    pub launcher_slot_count: ValOption,
    pub module_state: ValOption,
    pub capital_module: ValOption,
    pub overload_skill: ValOption,
    pub unusable_cap: ValOption,
    // Charges
    pub charge_group: ValOption,
    pub charge_parent_group: ValOption,
    pub charge_size: ValOption,
    pub charge_volume: ValOption,
    // Rigs
    pub rig_slot_count: ValOption,
    pub calibration: ValOption,
    pub rig_size: ValOption,
    // Services
    pub service_slot_count: ValOption,
    // T3 subsystems/stances
    pub subsystem_slot_count: ValOption,
    pub subsystem_slot_index: ValOption,
    pub ship_stance: ValOption,
    // Drones
    pub drone_bay_volume: ValOption,
    pub launched_drone_count: ValOption,
    pub drone_bandwidth: ValOption,
    pub unlaunchable_drone_slot: ValOption,
    pub unlaunchable_drone_bandwidth: ValOption,
    pub drone_group: ValOption,
    // Fighters
    pub fighter_bay_volume: ValOption,
    pub launched_fighter_count: ValOption,
    pub launched_light_fighter_count: ValOption,
    pub launched_heavy_fighter_count: ValOption,
    pub launched_support_fighter_count: ValOption,
    pub launched_st_light_fighter_count: ValOption,
    pub launched_st_heavy_fighter_count: ValOption,
    pub launched_st_support_fighter_count: ValOption,
    pub unlaunchable_fighter: ValOption,
    pub unlaunchable_light_fighter: ValOption,
    pub unlaunchable_heavy_fighter: ValOption,
    pub unlaunchable_support_fighter: ValOption,
    pub unlaunchable_st_light_fighter: ValOption,
    pub unlaunchable_st_heavy_fighter: ValOption,
    pub unlaunchable_st_support_fighter: ValOption,
    pub fighter_squad_size: ValOption,
    // Projection, destination side
    pub activation_blocked: ValOption,
    pub effect_stopper: ValOption,
    // Projection, source side
    pub projectee_filter: ValOption,
    pub assist_immunity: ValOption,
    pub offense_immunity: ValOption,
    pub resist_immunity: ValOption,
    // Sec zone
    pub sec_zone_fitted: ValOption,
    pub sec_zone_online: ValOption,
    pub sec_zone_active: ValOption,
    pub sec_zone_unonlineable: ValOption,
    pub sec_zone_unactivable: ValOption,
}
impl ValOptions {
    /// Initialize with all validations enabled.
    pub fn all_enabled() -> Self {
        Self {
            // Generic
            not_loaded_item: ValOption::new_enabled(),
            item_kind: ValOption::new_enabled(),
            skill_reqs: ValOption::new_enabled(),
            // Implants/boosters
            implant_slot_index: ValOption::new_enabled(),
            booster_slot_index: ValOption::new_enabled(),
            // Shared between mod-alike items
            cpu: ValOption::new_enabled(),
            powergrid: ValOption::new_enabled(),
            ship_limit: ValOption::new_enabled(),
            max_group_fitted: ValOption::new_enabled(),
            max_group_online: ValOption::new_enabled(),
            max_group_active: ValOption::new_enabled(),
            max_type_fitted: ValOption::new_enabled(),
            item_vs_ship_kind: ValOption::new_enabled(),
            // Modules
            high_slot_count: ValOption::new_enabled(),
            mid_slot_count: ValOption::new_enabled(),
            low_slot_count: ValOption::new_enabled(),
            turret_slot_count: ValOption::new_enabled(),
            launcher_slot_count: ValOption::new_enabled(),
            module_state: ValOption::new_enabled(),
            capital_module: ValOption::new_enabled(),
            overload_skill: ValOption::new_enabled(),
            unusable_cap: ValOption::new_enabled(),
            // Charges
            charge_group: ValOption::new_enabled(),
            charge_parent_group: ValOption::new_enabled(),
            charge_size: ValOption::new_enabled(),
            charge_volume: ValOption::new_enabled(),
            // Rigs
            rig_slot_count: ValOption::new_enabled(),
            calibration: ValOption::new_enabled(),
            rig_size: ValOption::new_enabled(),
            // Services
            service_slot_count: ValOption::new_enabled(),
            // T3 subsystems/stances
            subsystem_slot_count: ValOption::new_enabled(),
            subsystem_slot_index: ValOption::new_enabled(),
            ship_stance: ValOption::new_enabled(),
            // Drones
            drone_bay_volume: ValOption::new_enabled(),
            launched_drone_count: ValOption::new_enabled(),
            drone_bandwidth: ValOption::new_enabled(),
            unlaunchable_drone_slot: ValOption::new_enabled(),
            unlaunchable_drone_bandwidth: ValOption::new_enabled(),
            drone_group: ValOption::new_enabled(),
            // Fighters
            fighter_bay_volume: ValOption::new_enabled(),
            launched_fighter_count: ValOption::new_enabled(),
            launched_light_fighter_count: ValOption::new_enabled(),
            launched_heavy_fighter_count: ValOption::new_enabled(),
            launched_support_fighter_count: ValOption::new_enabled(),
            launched_st_light_fighter_count: ValOption::new_enabled(),
            launched_st_heavy_fighter_count: ValOption::new_enabled(),
            launched_st_support_fighter_count: ValOption::new_enabled(),
            unlaunchable_fighter: ValOption::new_enabled(),
            unlaunchable_light_fighter: ValOption::new_enabled(),
            unlaunchable_heavy_fighter: ValOption::new_enabled(),
            unlaunchable_support_fighter: ValOption::new_enabled(),
            unlaunchable_st_light_fighter: ValOption::new_enabled(),
            unlaunchable_st_heavy_fighter: ValOption::new_enabled(),
            unlaunchable_st_support_fighter: ValOption::new_enabled(),
            fighter_squad_size: ValOption::new_enabled(),
            // Projection, destination side
            activation_blocked: ValOption::new_enabled(),
            effect_stopper: ValOption::new_enabled(),
            // Projection, source side
            projectee_filter: ValOption::new_enabled(),
            assist_immunity: ValOption::new_enabled(),
            offense_immunity: ValOption::new_enabled(),
            resist_immunity: ValOption::new_enabled(),
            // Sec zone
            sec_zone_fitted: ValOption::new_enabled(),
            sec_zone_online: ValOption::new_enabled(),
            sec_zone_active: ValOption::new_enabled(),
            sec_zone_unonlineable: ValOption::new_enabled(),
            sec_zone_unactivable: ValOption::new_enabled(),
        }
    }
    /// Initialize with all validations disabled.
    pub fn all_disabled() -> Self {
        Self {
            // Generic
            not_loaded_item: ValOption::new_disabled(),
            item_kind: ValOption::new_disabled(),
            skill_reqs: ValOption::new_disabled(),
            // Implants/boosters
            implant_slot_index: ValOption::new_disabled(),
            booster_slot_index: ValOption::new_disabled(),
            // Shared between mod-alike items
            cpu: ValOption::new_disabled(),
            powergrid: ValOption::new_disabled(),
            ship_limit: ValOption::new_disabled(),
            max_group_fitted: ValOption::new_disabled(),
            max_group_online: ValOption::new_disabled(),
            max_group_active: ValOption::new_disabled(),
            max_type_fitted: ValOption::new_disabled(),
            item_vs_ship_kind: ValOption::new_disabled(),
            // Modules
            high_slot_count: ValOption::new_disabled(),
            mid_slot_count: ValOption::new_disabled(),
            low_slot_count: ValOption::new_disabled(),
            turret_slot_count: ValOption::new_disabled(),
            launcher_slot_count: ValOption::new_disabled(),
            module_state: ValOption::new_disabled(),
            capital_module: ValOption::new_disabled(),
            overload_skill: ValOption::new_disabled(),
            unusable_cap: ValOption::new_disabled(),
            // Charges
            charge_group: ValOption::new_disabled(),
            charge_parent_group: ValOption::new_disabled(),
            charge_size: ValOption::new_disabled(),
            charge_volume: ValOption::new_disabled(),
            // Rigs
            rig_slot_count: ValOption::new_disabled(),
            calibration: ValOption::new_disabled(),
            rig_size: ValOption::new_disabled(),
            // Services
            service_slot_count: ValOption::new_disabled(),
            // T3 subsystems/stances
            subsystem_slot_count: ValOption::new_disabled(),
            subsystem_slot_index: ValOption::new_disabled(),
            ship_stance: ValOption::new_disabled(),
            // Drones
            drone_bay_volume: ValOption::new_disabled(),
            launched_drone_count: ValOption::new_disabled(),
            drone_bandwidth: ValOption::new_disabled(),
            unlaunchable_drone_slot: ValOption::new_disabled(),
            unlaunchable_drone_bandwidth: ValOption::new_disabled(),
            drone_group: ValOption::new_disabled(),
            // Fighters
            fighter_bay_volume: ValOption::new_disabled(),
            launched_fighter_count: ValOption::new_disabled(),
            launched_light_fighter_count: ValOption::new_disabled(),
            launched_heavy_fighter_count: ValOption::new_disabled(),
            launched_support_fighter_count: ValOption::new_disabled(),
            launched_st_light_fighter_count: ValOption::new_disabled(),
            launched_st_heavy_fighter_count: ValOption::new_disabled(),
            launched_st_support_fighter_count: ValOption::new_disabled(),
            unlaunchable_fighter: ValOption::new_disabled(),
            unlaunchable_light_fighter: ValOption::new_disabled(),
            unlaunchable_heavy_fighter: ValOption::new_disabled(),
            unlaunchable_support_fighter: ValOption::new_disabled(),
            unlaunchable_st_light_fighter: ValOption::new_disabled(),
            unlaunchable_st_heavy_fighter: ValOption::new_disabled(),
            unlaunchable_st_support_fighter: ValOption::new_disabled(),
            fighter_squad_size: ValOption::new_disabled(),
            // Projection, destination side
            activation_blocked: ValOption::new_disabled(),
            effect_stopper: ValOption::new_disabled(),
            // Projection, source side
            projectee_filter: ValOption::new_disabled(),
            assist_immunity: ValOption::new_disabled(),
            offense_immunity: ValOption::new_disabled(),
            resist_immunity: ValOption::new_disabled(),
            // Sec zone
            sec_zone_fitted: ValOption::new_disabled(),
            sec_zone_online: ValOption::new_disabled(),
            sec_zone_active: ValOption::new_disabled(),
            sec_zone_unonlineable: ValOption::new_disabled(),
            sec_zone_unactivable: ValOption::new_disabled(),
        }
    }
}

/// Options for individual validation.
#[derive(Clone)]
pub struct ValOption {
    /// Controls if validation will be run or not.
    pub enabled: bool,
    /// Known failures or a validation.
    ///
    /// Every validation failure is attached to an item. Items listed here will not be returned as
    /// validation failures. If all validation's failures are known, it is passed.
    pub kfs: Vec<ItemId>,
}
impl ValOption {
    /// Initialize options with enabled flag on.
    pub fn new_enabled() -> Self {
        Self {
            enabled: true,
            kfs: Vec::new(),
        }
    }
    /// Initialize options with enabled flag off.
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            kfs: Vec::new(),
        }
    }
}

// Internal variant of validation options, with fit/item keys instead of IDs.
pub(crate) struct ValOptionsSolInt {
    pub(crate) options: ValOptionsInt,
    pub(crate) fit_keys: Vec<UFitKey>,
}
impl ValOptionsSolInt {
    pub(crate) fn from_pub(sol: &SolarSystem, pub_sol_opts: &ValOptionsSol) -> Self {
        Self {
            options: ValOptionsInt::from_pub(sol, &pub_sol_opts.options),
            fit_keys: pub_sol_opts
                .fit_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.fits.key_by_id(fit_id))
                .unique()
                .collect(),
        }
    }
}

pub(crate) struct ValOptionsInt {
    // Generic
    pub(in crate::svc::vast) not_loaded_item: ValOptionInt,
    pub(in crate::svc::vast) item_kind: ValOptionInt,
    pub(in crate::svc::vast) skill_reqs: ValOptionInt,
    // Implants/boosters
    pub(in crate::svc::vast) implant_slot_index: ValOptionInt,
    pub(in crate::svc::vast) booster_slot_index: ValOptionInt,
    // Shared between mod-alike items
    pub(in crate::svc::vast) cpu: ValOptionInt,
    pub(in crate::svc::vast) powergrid: ValOptionInt,
    pub(in crate::svc::vast) ship_limit: ValOptionInt,
    pub(in crate::svc::vast) max_group_fitted: ValOptionInt,
    pub(in crate::svc::vast) max_group_online: ValOptionInt,
    pub(in crate::svc::vast) max_group_active: ValOptionInt,
    pub(in crate::svc::vast) max_type_fitted: ValOptionInt,
    pub(in crate::svc::vast) item_vs_ship_kind: ValOptionInt,
    // Modules
    pub(in crate::svc::vast) high_slot_count: ValOptionInt,
    pub(in crate::svc::vast) mid_slot_count: ValOptionInt,
    pub(in crate::svc::vast) low_slot_count: ValOptionInt,
    pub(in crate::svc::vast) turret_slot_count: ValOptionInt,
    pub(in crate::svc::vast) launcher_slot_count: ValOptionInt,
    pub(in crate::svc::vast) module_state: ValOptionInt,
    pub(in crate::svc::vast) capital_module: ValOptionInt,
    pub(in crate::svc::vast) overload_skill: ValOptionInt,
    pub(in crate::svc::vast) unusable_cap: ValOptionInt,
    // Charges
    pub(in crate::svc::vast) charge_group: ValOptionInt,
    pub(in crate::svc::vast) charge_parent_group: ValOptionInt,
    pub(in crate::svc::vast) charge_size: ValOptionInt,
    pub(in crate::svc::vast) charge_volume: ValOptionInt,
    // Rigs
    pub(in crate::svc::vast) rig_slot_count: ValOptionInt,
    pub(in crate::svc::vast) calibration: ValOptionInt,
    pub(in crate::svc::vast) rig_size: ValOptionInt,
    // Services
    pub(in crate::svc::vast) service_slot_count: ValOptionInt,
    // T3 subsystems/stances
    pub(in crate::svc::vast) subsystem_slot_count: ValOptionInt,
    pub(in crate::svc::vast) subsystem_slot_index: ValOptionInt,
    pub(in crate::svc::vast) ship_stance: ValOptionInt,
    // Drones
    pub(in crate::svc::vast) drone_bay_volume: ValOptionInt,
    pub(in crate::svc::vast) launched_drone_count: ValOptionInt,
    pub(in crate::svc::vast) drone_bandwidth: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_drone_slot: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_drone_bandwidth: ValOptionInt,
    pub(in crate::svc::vast) drone_group: ValOptionInt,
    // Fighters
    pub(in crate::svc::vast) fighter_bay_volume: ValOptionInt,
    pub(in crate::svc::vast) launched_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_light_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_heavy_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_support_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_st_light_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_st_heavy_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) launched_st_support_fighter_count: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_light_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_heavy_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_support_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_st_light_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_st_heavy_fighter: ValOptionInt,
    pub(in crate::svc::vast) unlaunchable_st_support_fighter: ValOptionInt,
    pub(in crate::svc::vast) fighter_squad_size: ValOptionInt,
    // Projection, destination side
    pub(in crate::svc::vast) activation_blocked: ValOptionInt,
    pub(in crate::svc::vast) effect_stopper: ValOptionInt,
    // Projection, source side
    pub(in crate::svc::vast) projectee_filter: ValOptionInt,
    pub(in crate::svc::vast) assist_immunity: ValOptionInt,
    pub(in crate::svc::vast) offense_immunity: ValOptionInt,
    pub(in crate::svc::vast) resist_immunity: ValOptionInt,
    // Sec zone
    pub(in crate::svc::vast) sec_zone_fitted: ValOptionInt,
    pub(in crate::svc::vast) sec_zone_online: ValOptionInt,
    pub(in crate::svc::vast) sec_zone_active: ValOptionInt,
    pub(in crate::svc::vast) sec_zone_unonlineable: ValOptionInt,
    pub(in crate::svc::vast) sec_zone_unactivable: ValOptionInt,
}
impl ValOptionsInt {
    pub(crate) fn from_pub(sol: &SolarSystem, pub_opts: &ValOptions) -> Self {
        Self {
            // Generic
            not_loaded_item: ValOptionInt::from_pub(sol, &pub_opts.not_loaded_item),
            item_kind: ValOptionInt::from_pub(sol, &pub_opts.item_kind),
            skill_reqs: ValOptionInt::from_pub(sol, &pub_opts.skill_reqs),
            // Implants/boosters
            implant_slot_index: ValOptionInt::from_pub(sol, &pub_opts.implant_slot_index),
            booster_slot_index: ValOptionInt::from_pub(sol, &pub_opts.booster_slot_index),
            // Shared between mod-alike items
            cpu: ValOptionInt::from_pub(sol, &pub_opts.cpu),
            powergrid: ValOptionInt::from_pub(sol, &pub_opts.powergrid),
            ship_limit: ValOptionInt::from_pub(sol, &pub_opts.ship_limit),
            max_group_fitted: ValOptionInt::from_pub(sol, &pub_opts.max_group_fitted),
            max_group_online: ValOptionInt::from_pub(sol, &pub_opts.max_group_online),
            max_group_active: ValOptionInt::from_pub(sol, &pub_opts.max_group_active),
            max_type_fitted: ValOptionInt::from_pub(sol, &pub_opts.max_type_fitted),
            item_vs_ship_kind: ValOptionInt::from_pub(sol, &pub_opts.item_vs_ship_kind),
            // Modules
            high_slot_count: ValOptionInt::from_pub(sol, &pub_opts.high_slot_count),
            mid_slot_count: ValOptionInt::from_pub(sol, &pub_opts.mid_slot_count),
            low_slot_count: ValOptionInt::from_pub(sol, &pub_opts.low_slot_count),
            turret_slot_count: ValOptionInt::from_pub(sol, &pub_opts.turret_slot_count),
            launcher_slot_count: ValOptionInt::from_pub(sol, &pub_opts.launcher_slot_count),
            module_state: ValOptionInt::from_pub(sol, &pub_opts.module_state),
            capital_module: ValOptionInt::from_pub(sol, &pub_opts.capital_module),
            overload_skill: ValOptionInt::from_pub(sol, &pub_opts.overload_skill),
            unusable_cap: ValOptionInt::from_pub(sol, &pub_opts.unusable_cap),
            // Charges
            charge_group: ValOptionInt::from_pub(sol, &pub_opts.charge_group),
            charge_parent_group: ValOptionInt::from_pub(sol, &pub_opts.charge_parent_group),
            charge_size: ValOptionInt::from_pub(sol, &pub_opts.charge_size),
            charge_volume: ValOptionInt::from_pub(sol, &pub_opts.charge_volume),
            // Rigs
            rig_slot_count: ValOptionInt::from_pub(sol, &pub_opts.rig_slot_count),
            calibration: ValOptionInt::from_pub(sol, &pub_opts.calibration),
            rig_size: ValOptionInt::from_pub(sol, &pub_opts.rig_size),
            // Services
            service_slot_count: ValOptionInt::from_pub(sol, &pub_opts.service_slot_count),
            // T3 subsystems/stances
            subsystem_slot_count: ValOptionInt::from_pub(sol, &pub_opts.subsystem_slot_count),
            subsystem_slot_index: ValOptionInt::from_pub(sol, &pub_opts.subsystem_slot_index),
            ship_stance: ValOptionInt::from_pub(sol, &pub_opts.ship_stance),
            // Drones
            drone_bay_volume: ValOptionInt::from_pub(sol, &pub_opts.drone_bay_volume),
            launched_drone_count: ValOptionInt::from_pub(sol, &pub_opts.launched_drone_count),
            drone_bandwidth: ValOptionInt::from_pub(sol, &pub_opts.drone_bandwidth),
            unlaunchable_drone_slot: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_drone_slot),
            unlaunchable_drone_bandwidth: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_drone_bandwidth),
            drone_group: ValOptionInt::from_pub(sol, &pub_opts.drone_group),
            // Fighters
            fighter_bay_volume: ValOptionInt::from_pub(sol, &pub_opts.fighter_bay_volume),
            launched_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_fighter_count),
            launched_light_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_light_fighter_count),
            launched_heavy_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_heavy_fighter_count),
            launched_support_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_support_fighter_count),
            launched_st_light_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_st_light_fighter_count),
            launched_st_heavy_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_st_heavy_fighter_count),
            launched_st_support_fighter_count: ValOptionInt::from_pub(sol, &pub_opts.launched_st_support_fighter_count),
            unlaunchable_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_fighter),
            unlaunchable_light_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_light_fighter),
            unlaunchable_heavy_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_heavy_fighter),
            unlaunchable_support_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_support_fighter),
            unlaunchable_st_light_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_st_light_fighter),
            unlaunchable_st_heavy_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_st_heavy_fighter),
            unlaunchable_st_support_fighter: ValOptionInt::from_pub(sol, &pub_opts.unlaunchable_st_support_fighter),
            fighter_squad_size: ValOptionInt::from_pub(sol, &pub_opts.fighter_squad_size),
            // Projection, destination side
            activation_blocked: ValOptionInt::from_pub(sol, &pub_opts.activation_blocked),
            effect_stopper: ValOptionInt::from_pub(sol, &pub_opts.effect_stopper),
            // Projection, source side
            projectee_filter: ValOptionInt::from_pub(sol, &pub_opts.projectee_filter),
            assist_immunity: ValOptionInt::from_pub(sol, &pub_opts.assist_immunity),
            offense_immunity: ValOptionInt::from_pub(sol, &pub_opts.offense_immunity),
            resist_immunity: ValOptionInt::from_pub(sol, &pub_opts.resist_immunity),
            // Sec zone
            sec_zone_fitted: ValOptionInt::from_pub(sol, &pub_opts.sec_zone_fitted),
            sec_zone_online: ValOptionInt::from_pub(sol, &pub_opts.sec_zone_online),
            sec_zone_active: ValOptionInt::from_pub(sol, &pub_opts.sec_zone_active),
            sec_zone_unonlineable: ValOptionInt::from_pub(sol, &pub_opts.sec_zone_unonlineable),
            sec_zone_unactivable: ValOptionInt::from_pub(sol, &pub_opts.sec_zone_unactivable),
        }
    }
}

pub(in crate::svc::vast) struct ValOptionInt {
    pub(in crate::svc::vast) enabled: bool,
    pub(in crate::svc::vast) kfs: RSet<UItemKey>,
}
impl ValOptionInt {
    fn from_pub(sol: &SolarSystem, pub_opt: &ValOption) -> Self {
        Self {
            enabled: pub_opt.enabled,
            kfs: pub_opt
                .kfs
                .iter()
                .filter_map(|item_id| sol.u_data.items.key_by_id(item_id))
                .unique()
                .collect(),
        }
    }
}
