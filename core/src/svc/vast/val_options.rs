use itertools::Itertools;

use crate::{
    sol::SolarSystem,
    ud::{FitId, ItemId, UFitId, UItemId},
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
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Generic
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails for any items which are not loaded. Items can become not loaded when they were added
    /// to a fit, but current data source does not have an EVE item with corresponding type ID.
    pub not_loaded_item: ValOption,
    /// Any EVE item usually can be represented by a single item kind in the lib. For example, an
    /// item from Implant category with "boosterness" attribute is a booster. This validation checks
    /// relations between user-defined item kind and item kind detected for a backing EVE item.
    pub item_kind: ValOption,
    /// Fails when a direct skill requirement is not satisfied for an item.
    pub skill_reqs: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Implants/boosters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when multiple implants attempt to take the same slot.
    pub implant_slot_index: ValOption,
    /// Fails when multiple boosters attempt to take the same slot.
    pub booster_slot_index: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Shared between mod-alike items
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when items take more CPU than ship can produce.
    pub cpu: ValOption,
    /// Fails when items take more PG than ship can produce.
    pub powergrid: ValOption,
    /// When a fit has any items which can be fit to specific set of ships (identified by ship list
    /// and ship group list), and ship does not fall into it, this validation is failed for those
    /// items.
    pub ship_limit: ValOption,
    /// When an item has limit on how many items from its group can be fitted, and count of fitted
    /// items exceeds that, this validation fails.
    pub max_group_fitted: ValOption,
    /// When an item has limit on how many items from its group can be online, and count of online
    /// items exceeds that, this validation fails.
    pub max_group_online: ValOption,
    /// When an item has limit on how many items from its group can be active, and count of active
    /// items exceeds that, this validation fails.
    pub max_group_active: ValOption,
    /// When an item has limit on how many items with the same type ID can be fitted, and count of
    /// fitted items exceeds that, this validation fails.
    pub max_type_fitted: ValOption,
    /// Checks that structure items are not fit to a ship fit, and ship items are not fit to a
    /// structure fit. Type of fit is defined by its ship kind.
    pub item_vs_ship_kind: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Modules
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// If any of high slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub high_slot_count: ValOption,
    /// If any of medium slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub mid_slot_count: ValOption,
    /// If any of low slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub low_slot_count: ValOption,
    /// If count of taken turret slots is higher than ship provides, this validation fails for all
    /// modules which need a turret slot.
    pub turret_slot_count: ValOption,
    /// If count of taken launcher slots is higher than ship provides, this validation fails for all
    /// modules which need a launcher slot.
    pub launcher_slot_count: ValOption,
    /// If any module has state higher than it supports (e.g. active bulkhead), this validation
    /// fails.
    pub module_state: ValOption,
    /// Fails when any capital modules (large-volume modules) are fit to subcapital ships.
    pub capital_module: ValOption,
    /// Fails when fit has any items overloaded, and overload skill requirement is not satisfied.
    pub overload_skill: ValOption,
    /// Fails when any item consumes more cap than ship has. Only on-fit items which consume cap are
    /// considered for this, anything else (e.g. incoming neutralizers) are ignored.
    pub unusable_cap: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Charges
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Some modules restrict charges which can be loaded into them by group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub charge_group: ValOption,
    /// Some charges restrict into which modules they can be loaded by module group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub charge_parent_group: ValOption,
    /// Some charges and modules have charge size set. When a module specifies it, and has a charge
    /// without size or with mismatching size loaded, this validation fails for the charge.
    pub charge_size: ValOption,
    /// Fails when volume of a single charge is larger than capacity of a module it's loaded into.
    pub charge_volume: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Rigs
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more rigs than ship has rig slots.
    pub rig_slot_count: ValOption,
    /// Fails when rigs take more calibration than ship can produce.
    pub calibration: ValOption,
    /// Ships and rigs specify rig size; when those mismatch, this validation fails for rigs.
    pub rig_size: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Services
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more services than ship/structure has service slots.
    pub service_slot_count: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // T3 subsystems/stances
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more subsystems than ship has subsystem slots.
    pub subsystem_slot_count: ValOption,
    /// Fails when multiple subsystems attempt to take the same slot.
    pub subsystem_slot_index: ValOption,
    /// Fails when a ship which can't have a stance but has one.
    pub ship_stance: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Drones
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when drones take more volume than ship's drone bay has.
    pub drone_bay_volume: ValOption,
    /// Fails when fit has more in-space drones than ship supports.
    pub launched_drone_count: ValOption,
    /// Fails when in-space drones take more bandwidth than ship provides.
    pub drone_bandwidth: ValOption,
    /// Fails when fit has any drones when ship supports none.
    pub unlaunchable_drone_slot: ValOption,
    /// Fails when fit has any drones which take more bandwidth than ship provides.
    pub unlaunchable_drone_bandwidth: ValOption,
    /// Ship can limit which drone groups can be put into its drone bay. If it does, and drones from
    /// mismatching groups are fit, this validation fails for those drones.
    pub drone_group: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Fighters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fighters take more volume than ship's fighter bay has.
    pub fighter_bay_volume: ValOption,
    /// Fails when fit has more in-space fighters than ship supports.
    pub launched_fighter_count: ValOption,
    /// Fails when fit has more in-space light fighters than ship supports.
    pub launched_light_fighter_count: ValOption,
    /// Fails when fit has more in-space heavy fighters than ship supports.
    pub launched_heavy_fighter_count: ValOption,
    /// Fails when fit has more in-space support fighters than ship supports.
    pub launched_support_fighter_count: ValOption,
    /// Fails when fit has more in-space standup light fighters than ship supports.
    pub launched_st_light_fighter_count: ValOption,
    /// Fails when fit has more in-space standup heavy fighters than ship supports.
    pub launched_st_heavy_fighter_count: ValOption,
    /// Fails when fit has more in-space standup support fighters than ship supports.
    pub launched_st_support_fighter_count: ValOption,
    /// Fails when fit has any fighters when ship supports none.
    pub unlaunchable_fighter: ValOption,
    /// Fails when fit has any light fighters when ship supports none.
    pub unlaunchable_light_fighter: ValOption,
    /// Fails when fit has any heavy fighters when ship supports none.
    pub unlaunchable_heavy_fighter: ValOption,
    /// Fails when fit has any support fighters when ship supports none.
    pub unlaunchable_support_fighter: ValOption,
    /// Fails when fit has any standup light fighters when ship supports none.
    pub unlaunchable_st_light_fighter: ValOption,
    /// Fails when fit has any standup heavy fighters when ship supports none.
    pub unlaunchable_st_heavy_fighter: ValOption,
    /// Fails when fit has any standup support fighters when ship supports none.
    pub unlaunchable_st_support_fighter: ValOption,
    /// Fails for fighter squads which have more fighters than squad supports.
    pub fighter_squad_size: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, destination side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when any modules are active but their activation is blocked (e.g. scrambled MWDs).
    pub activation_blocked: ValOption,
    /// Fails when any items have running effects which are stopped by external factors (e.g.
    /// scrambled fighter MWD).
    pub effect_stopper: ValOption,
    /// When a cloak is active and something blocks it (weather, modules incompatible with cloaking
    /// like sieges, multiple cloaks fit to ship), this validation fails.
    pub cloaking_blocked: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, source side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when item defines which targets it can be applied to, but some of its targets do not
    /// belong to it.
    pub projectee_filter: ValOption,
    /// Fails when item is marked as assistive, and is applied to a target which is immune to
    /// assistance.
    pub assist_immunity: ValOption,
    /// Fails when item is marked as offensive, and is applied to a target which is immune to
    /// offense.
    pub offense_immunity: ValOption,
    /// Fails when item's effect can be resisted, and is applied to a target which completely
    /// resists its effect.
    pub resist_immunity: ValOption,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Sec zone
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when some items are not allowed to be fitted in current sol security zone.
    pub sec_zone_fitted: ValOption,
    /// Fails when some items are not allowed to be online in current sol security zone.
    pub sec_zone_online: ValOption,
    /// Fails when some items are not allowed to be active in current sol security zone.
    pub sec_zone_active: ValOption,
    /// Fails when fit has items which cannot be onlined in current sol security zone.
    pub sec_zone_unonlineable: ValOption,
    /// Fails when fit has items which cannot be activated in current sol security zone.
    pub sec_zone_unactivable: ValOption,
    /// Fails when some effects are not allowed to run in current sol security zone.
    pub sec_zone_effect: ValOption,
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
            cloaking_blocked: ValOption::new_enabled(),
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
            sec_zone_effect: ValOption::new_enabled(),
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
            cloaking_blocked: ValOption::new_disabled(),
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
            sec_zone_effect: ValOption::new_disabled(),
        }
    }
}

/// Controls if validation will be run or not.
#[derive(Clone)]
pub enum ValOption {
    Enabled(ValOptionEnabledOptions),
    Disabled,
}
impl ValOption {
    /// Initialize options with enabled flag on.
    pub fn new_enabled() -> Self {
        Self::Enabled(Default::default())
    }
    /// Initialize options with enabled flag off.
    pub fn new_disabled() -> Self {
        Self::Disabled
    }
}

#[derive(Clone, Default)]
pub struct ValOptionEnabledOptions {
    /// Known failures of a validation.
    ///
    /// Every validation failure is attached to an item. Items listed here will not be returned as
    /// validation failures. If all validation's failures are known, it is passed.
    pub kfs: Vec<ItemId>,
}
impl ValOptionEnabledOptions {
    pub fn new() -> Self {
        Self { kfs: Vec::default() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal variant, with fit/item UIDs instead of external IDs
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct ValOptionsSolInt {
    pub(crate) options: ValOptionsInt,
    pub(crate) fit_uids: Vec<UFitId>,
}
impl ValOptionsSolInt {
    pub(crate) fn from_pub(pub_sol_opts: &ValOptionsSol, sol: &SolarSystem) -> Self {
        Self {
            options: ValOptionsInt::from_pub(&pub_sol_opts.options, sol),
            fit_uids: pub_sol_opts
                .fit_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.fits.iid_by_xid(fit_id))
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
    pub(in crate::svc::vast) cloaking_blocked: ValOptionInt,
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
    pub(in crate::svc::vast) sec_zone_effect: ValOptionInt,
}
impl ValOptionsInt {
    pub(crate) fn from_pub(pub_opts: &ValOptions, sol: &SolarSystem) -> Self {
        Self {
            // Generic
            not_loaded_item: ValOptionInt::from_pub(&pub_opts.not_loaded_item, sol),
            item_kind: ValOptionInt::from_pub(&pub_opts.item_kind, sol),
            skill_reqs: ValOptionInt::from_pub(&pub_opts.skill_reqs, sol),
            // Implants/boosters
            implant_slot_index: ValOptionInt::from_pub(&pub_opts.implant_slot_index, sol),
            booster_slot_index: ValOptionInt::from_pub(&pub_opts.booster_slot_index, sol),
            // Shared between mod-alike items
            cpu: ValOptionInt::from_pub(&pub_opts.cpu, sol),
            powergrid: ValOptionInt::from_pub(&pub_opts.powergrid, sol),
            ship_limit: ValOptionInt::from_pub(&pub_opts.ship_limit, sol),
            max_group_fitted: ValOptionInt::from_pub(&pub_opts.max_group_fitted, sol),
            max_group_online: ValOptionInt::from_pub(&pub_opts.max_group_online, sol),
            max_group_active: ValOptionInt::from_pub(&pub_opts.max_group_active, sol),
            max_type_fitted: ValOptionInt::from_pub(&pub_opts.max_type_fitted, sol),
            item_vs_ship_kind: ValOptionInt::from_pub(&pub_opts.item_vs_ship_kind, sol),
            // Modules
            high_slot_count: ValOptionInt::from_pub(&pub_opts.high_slot_count, sol),
            mid_slot_count: ValOptionInt::from_pub(&pub_opts.mid_slot_count, sol),
            low_slot_count: ValOptionInt::from_pub(&pub_opts.low_slot_count, sol),
            turret_slot_count: ValOptionInt::from_pub(&pub_opts.turret_slot_count, sol),
            launcher_slot_count: ValOptionInt::from_pub(&pub_opts.launcher_slot_count, sol),
            module_state: ValOptionInt::from_pub(&pub_opts.module_state, sol),
            capital_module: ValOptionInt::from_pub(&pub_opts.capital_module, sol),
            overload_skill: ValOptionInt::from_pub(&pub_opts.overload_skill, sol),
            unusable_cap: ValOptionInt::from_pub(&pub_opts.unusable_cap, sol),
            // Charges
            charge_group: ValOptionInt::from_pub(&pub_opts.charge_group, sol),
            charge_parent_group: ValOptionInt::from_pub(&pub_opts.charge_parent_group, sol),
            charge_size: ValOptionInt::from_pub(&pub_opts.charge_size, sol),
            charge_volume: ValOptionInt::from_pub(&pub_opts.charge_volume, sol),
            // Rigs
            rig_slot_count: ValOptionInt::from_pub(&pub_opts.rig_slot_count, sol),
            calibration: ValOptionInt::from_pub(&pub_opts.calibration, sol),
            rig_size: ValOptionInt::from_pub(&pub_opts.rig_size, sol),
            // Services
            service_slot_count: ValOptionInt::from_pub(&pub_opts.service_slot_count, sol),
            // T3 subsystems/stances
            subsystem_slot_count: ValOptionInt::from_pub(&pub_opts.subsystem_slot_count, sol),
            subsystem_slot_index: ValOptionInt::from_pub(&pub_opts.subsystem_slot_index, sol),
            ship_stance: ValOptionInt::from_pub(&pub_opts.ship_stance, sol),
            // Drones
            drone_bay_volume: ValOptionInt::from_pub(&pub_opts.drone_bay_volume, sol),
            launched_drone_count: ValOptionInt::from_pub(&pub_opts.launched_drone_count, sol),
            drone_bandwidth: ValOptionInt::from_pub(&pub_opts.drone_bandwidth, sol),
            unlaunchable_drone_slot: ValOptionInt::from_pub(&pub_opts.unlaunchable_drone_slot, sol),
            unlaunchable_drone_bandwidth: ValOptionInt::from_pub(&pub_opts.unlaunchable_drone_bandwidth, sol),
            drone_group: ValOptionInt::from_pub(&pub_opts.drone_group, sol),
            // Fighters
            fighter_bay_volume: ValOptionInt::from_pub(&pub_opts.fighter_bay_volume, sol),
            launched_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_fighter_count, sol),
            launched_light_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_light_fighter_count, sol),
            launched_heavy_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_heavy_fighter_count, sol),
            launched_support_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_support_fighter_count, sol),
            launched_st_light_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_st_light_fighter_count, sol),
            launched_st_heavy_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_st_heavy_fighter_count, sol),
            launched_st_support_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_st_support_fighter_count, sol),
            unlaunchable_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_fighter, sol),
            unlaunchable_light_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_light_fighter, sol),
            unlaunchable_heavy_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_heavy_fighter, sol),
            unlaunchable_support_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_support_fighter, sol),
            unlaunchable_st_light_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_st_light_fighter, sol),
            unlaunchable_st_heavy_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_st_heavy_fighter, sol),
            unlaunchable_st_support_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_st_support_fighter, sol),
            fighter_squad_size: ValOptionInt::from_pub(&pub_opts.fighter_squad_size, sol),
            // Projection, destination side
            activation_blocked: ValOptionInt::from_pub(&pub_opts.activation_blocked, sol),
            effect_stopper: ValOptionInt::from_pub(&pub_opts.effect_stopper, sol),
            cloaking_blocked: ValOptionInt::from_pub(&pub_opts.cloaking_blocked, sol),
            // Projection, source side
            projectee_filter: ValOptionInt::from_pub(&pub_opts.projectee_filter, sol),
            assist_immunity: ValOptionInt::from_pub(&pub_opts.assist_immunity, sol),
            offense_immunity: ValOptionInt::from_pub(&pub_opts.offense_immunity, sol),
            resist_immunity: ValOptionInt::from_pub(&pub_opts.resist_immunity, sol),
            // Sec zone
            sec_zone_fitted: ValOptionInt::from_pub(&pub_opts.sec_zone_fitted, sol),
            sec_zone_online: ValOptionInt::from_pub(&pub_opts.sec_zone_online, sol),
            sec_zone_active: ValOptionInt::from_pub(&pub_opts.sec_zone_active, sol),
            sec_zone_unonlineable: ValOptionInt::from_pub(&pub_opts.sec_zone_unonlineable, sol),
            sec_zone_unactivable: ValOptionInt::from_pub(&pub_opts.sec_zone_unactivable, sol),
            sec_zone_effect: ValOptionInt::from_pub(&pub_opts.sec_zone_effect, sol),
        }
    }
}

pub(in crate::svc::vast) enum ValOptionInt {
    Enabled(ValOptionEnabledOptionsInt),
    Disabled,
}
impl ValOptionInt {
    fn from_pub(pub_opt: &ValOption, sol: &SolarSystem) -> Self {
        match pub_opt {
            ValOption::Enabled(pub_opt) => Self::Enabled(ValOptionEnabledOptionsInt {
                kfs: pub_opt
                    .kfs
                    .iter()
                    .filter_map(|item_id| sol.u_data.items.iid_by_xid(item_id))
                    .unique()
                    .collect(),
            }),
            ValOption::Disabled => Self::Disabled,
        }
    }
}

pub(in crate::svc::vast) struct ValOptionEnabledOptionsInt {
    pub(in crate::svc::vast) kfs: RSet<UItemId>,
}
