use itertools::Itertools;

use crate::{
    misc::DefOptionExt,
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

/// Validation options.
#[derive(Clone)]
pub struct ValOptions {
    /// True to have all validations enabled by default, false to have them disabled.
    pub default: bool,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Generic
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails for any items which are not loaded. Items can become not loaded when they were added
    /// to a fit, but current data source does not have an EVE item with corresponding type ID.
    pub not_loaded_item: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Any EVE item usually can be represented by a single item kind in the lib. For example, an
    /// item from Implant category with "boosterness" attribute is a booster. This validation checks
    /// relations between user-defined item kind and item kind detected for a backing EVE item.
    pub item_kind: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when a direct skill requirement is not satisfied for an item.
    pub skill_reqs: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Implants/boosters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when multiple implants attempt to take the same slot.
    pub implant_slot_index: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when multiple boosters attempt to take the same slot.
    pub booster_slot_index: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Shared between mod-alike items
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when items take more CPU than ship can produce.
    pub cpu: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when items take more PG than ship can produce.
    pub powergrid: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When a fit has any items which can be fit to specific set of ships (identified by ship list
    /// and ship group list), and ship does not fall into it, this validation is failed for those
    /// items.
    pub ship_limit: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When an item has limit on how many items from its group can be fitted, and count of fitted
    /// items exceeds that, this validation fails.
    pub max_group_fitted: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When an item has limit on how many items from its group can be online, and count of online
    /// items exceeds that, this validation fails.
    pub max_group_online: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When an item has limit on how many items from its group can be active, and count of active
    /// items exceeds that, this validation fails.
    pub max_group_active: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When an item has limit on how many items with the same type ID can be fitted, and count of
    /// fitted items exceeds that, this validation fails.
    pub max_type_fitted: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Checks that structure items are not fit to a ship fit, and ship items are not fit to a
    /// structure fit. Type of fit is defined by its ship kind.
    pub item_vs_ship_kind: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Modules
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// If any of high slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub high_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// If any of medium slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub mid_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// If any of low slot modules occupy slots with indices higher than ship supports, this
    /// validation fails, only for those modules.
    pub low_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// If count of taken turret slots is higher than ship provides, this validation fails for all
    /// modules which need a turret slot.
    pub turret_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// If count of taken launcher slots is higher than ship provides, this validation fails for all
    /// modules which need a launcher slot.
    pub launcher_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// If any module has state higher than it supports (e.g. active bulkhead), this validation
    /// fails.
    pub module_state: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when any capital modules (large-volume modules) are fit to subcapital ships.
    pub capital_module: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any items overloaded, and overload skill requirement is not satisfied.
    pub overload_skill: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when any item consumes more cap than ship has. Only on-fit items which consume cap are
    /// considered for this, anything else (e.g. incoming neutralizers) are ignored.
    pub unusable_cap: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Charges
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Some modules restrict charges which can be loaded into them by group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub charge_group: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Some charges restrict into which modules they can be loaded by module group; if charge from
    /// disallowed group is loaded, validation fails for charge.
    pub charge_parent_group: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Some charges and modules have charge size set. When a module specifies it, and has a charge
    /// without size or with mismatching size loaded, this validation fails for the charge.
    pub charge_size: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when volume of a single charge is larger than capacity of a module it's loaded into.
    pub charge_volume: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Rigs
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more rigs than ship has rig slots.
    pub rig_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when rigs take more calibration than ship can produce.
    pub calibration: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Ships and rigs specify rig size; when those mismatch, this validation fails for rigs.
    pub rig_size: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Services
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more services than ship/structure has service slots.
    pub service_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // T3 subsystems/stances
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fit has more subsystems than ship has subsystem slots.
    pub subsystem_slot_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when multiple subsystems attempt to take the same slot.
    pub subsystem_slot_index: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when a ship which can't have a stance but has one.
    pub ship_stance: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Drones
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when drones take more volume than ship's drone bay has.
    pub drone_bay_volume: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space drones than ship supports.
    pub launched_drone_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when in-space drones take more bandwidth than ship provides.
    pub drone_bandwidth: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any drones when ship supports none.
    pub unlaunchable_drone_slot: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any drones which take more bandwidth than ship provides.
    pub unlaunchable_drone_bandwidth: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Ship can limit which drone groups can be put into its drone bay. If it does, and drones from
    /// mismatching groups are fit, this validation fails for those drones.
    pub drone_group: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Fighters
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when fighters take more volume than ship's fighter bay has.
    pub fighter_bay_volume: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space fighters than ship supports.
    pub launched_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space light fighters than ship supports.
    pub launched_light_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space heavy fighters than ship supports.
    pub launched_heavy_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space support fighters than ship supports.
    pub launched_support_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space standup light fighters than ship supports.
    pub launched_st_light_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space standup heavy fighters than ship supports.
    pub launched_st_heavy_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has more in-space standup support fighters than ship supports.
    pub launched_st_support_fighter_count: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any fighters when ship supports none.
    pub unlaunchable_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any light fighters when ship supports none.
    pub unlaunchable_light_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any heavy fighters when ship supports none.
    pub unlaunchable_heavy_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any support fighters when ship supports none.
    pub unlaunchable_support_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any standup light fighters when ship supports none.
    pub unlaunchable_st_light_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any standup heavy fighters when ship supports none.
    pub unlaunchable_st_heavy_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has any standup support fighters when ship supports none.
    pub unlaunchable_st_support_fighter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails for fighter squads which have more fighters than squad supports.
    pub fighter_squad_size: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, destination side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when any modules are active but their activation is blocked (e.g. scrambled MWDs).
    pub activation_blocked: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when any items have running effects which are stopped by external factors (e.g.
    /// scrambled fighter MWD).
    pub effect_stopper: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// When a cloak is active and something blocks it (weather, modules incompatible with cloaking
    /// like sieges, multiple cloaks fit to ship), this validation fails.
    pub cloaking_blocked: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Projection, source side
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when item defines which targets it can be applied to, but some of its targets do not
    /// belong to it.
    pub projectee_filter: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when item is marked as assistive, and is applied to a target which is immune to
    /// assistance.
    pub assist_immunity: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when item is marked as offensive, and is applied to a target which is immune to
    /// offense.
    pub offense_immunity: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when item's effect can be resisted, and is applied to a target which completely
    /// resists its effect.
    pub resist_immunity: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Sec zone
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Fails when some items are not allowed to be fitted in current sol security zone.
    pub sec_zone_fitted: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when some items are not allowed to be online in current sol security zone.
    pub sec_zone_online: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when some items are not allowed to be active in current sol security zone.
    pub sec_zone_active: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has items which cannot be onlined in current sol security zone.
    pub sec_zone_unonlineable: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when fit has items which cannot be activated in current sol security zone.
    pub sec_zone_unactivable: DefOptionExt<ValEnabled> = DefOptionExt::Default,
    /// Fails when some effects are not allowed to run in current sol security zone.
    pub sec_zone_effect: DefOptionExt<ValEnabled> = DefOptionExt::Default,
}

#[derive(Clone, Default)]
pub struct ValEnabled {
    /// Known failures of a validation.
    ///
    /// Every validation failure is attached to an item. Items listed here will not be returned as
    /// validation failures. If all validation's failures are known, it is passed.
    pub kfs: Vec<ItemId>,
}
impl ValEnabled {
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
                .filter_map(|fit_id| sol.u_data.fits.int_id_by_ext_id(fit_id))
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
            not_loaded_item: ValOptionInt::from_pub(&pub_opts.not_loaded_item, pub_opts.default, sol),
            item_kind: ValOptionInt::from_pub(&pub_opts.item_kind, pub_opts.default, sol),
            skill_reqs: ValOptionInt::from_pub(&pub_opts.skill_reqs, pub_opts.default, sol),
            // Implants/boosters
            implant_slot_index: ValOptionInt::from_pub(&pub_opts.implant_slot_index, pub_opts.default, sol),
            booster_slot_index: ValOptionInt::from_pub(&pub_opts.booster_slot_index, pub_opts.default, sol),
            // Shared between mod-alike items
            cpu: ValOptionInt::from_pub(&pub_opts.cpu, pub_opts.default, sol),
            powergrid: ValOptionInt::from_pub(&pub_opts.powergrid, pub_opts.default, sol),
            ship_limit: ValOptionInt::from_pub(&pub_opts.ship_limit, pub_opts.default, sol),
            max_group_fitted: ValOptionInt::from_pub(&pub_opts.max_group_fitted, pub_opts.default, sol),
            max_group_online: ValOptionInt::from_pub(&pub_opts.max_group_online, pub_opts.default, sol),
            max_group_active: ValOptionInt::from_pub(&pub_opts.max_group_active, pub_opts.default, sol),
            max_type_fitted: ValOptionInt::from_pub(&pub_opts.max_type_fitted, pub_opts.default, sol),
            item_vs_ship_kind: ValOptionInt::from_pub(&pub_opts.item_vs_ship_kind, pub_opts.default, sol),
            // Modules
            high_slot_count: ValOptionInt::from_pub(&pub_opts.high_slot_count, pub_opts.default, sol),
            mid_slot_count: ValOptionInt::from_pub(&pub_opts.mid_slot_count, pub_opts.default, sol),
            low_slot_count: ValOptionInt::from_pub(&pub_opts.low_slot_count, pub_opts.default, sol),
            turret_slot_count: ValOptionInt::from_pub(&pub_opts.turret_slot_count, pub_opts.default, sol),
            launcher_slot_count: ValOptionInt::from_pub(&pub_opts.launcher_slot_count, pub_opts.default, sol),
            module_state: ValOptionInt::from_pub(&pub_opts.module_state, pub_opts.default, sol),
            capital_module: ValOptionInt::from_pub(&pub_opts.capital_module, pub_opts.default, sol),
            overload_skill: ValOptionInt::from_pub(&pub_opts.overload_skill, pub_opts.default, sol),
            unusable_cap: ValOptionInt::from_pub(&pub_opts.unusable_cap, pub_opts.default, sol),
            // Charges
            charge_group: ValOptionInt::from_pub(&pub_opts.charge_group, pub_opts.default, sol),
            charge_parent_group: ValOptionInt::from_pub(&pub_opts.charge_parent_group, pub_opts.default, sol),
            charge_size: ValOptionInt::from_pub(&pub_opts.charge_size, pub_opts.default, sol),
            charge_volume: ValOptionInt::from_pub(&pub_opts.charge_volume, pub_opts.default, sol),
            // Rigs
            rig_slot_count: ValOptionInt::from_pub(&pub_opts.rig_slot_count, pub_opts.default, sol),
            calibration: ValOptionInt::from_pub(&pub_opts.calibration, pub_opts.default, sol),
            rig_size: ValOptionInt::from_pub(&pub_opts.rig_size, pub_opts.default, sol),
            // Services
            service_slot_count: ValOptionInt::from_pub(&pub_opts.service_slot_count, pub_opts.default, sol),
            // T3 subsystems/stances
            subsystem_slot_count: ValOptionInt::from_pub(&pub_opts.subsystem_slot_count, pub_opts.default, sol),
            subsystem_slot_index: ValOptionInt::from_pub(&pub_opts.subsystem_slot_index, pub_opts.default, sol),
            ship_stance: ValOptionInt::from_pub(&pub_opts.ship_stance, pub_opts.default, sol),
            // Drones
            drone_bay_volume: ValOptionInt::from_pub(&pub_opts.drone_bay_volume, pub_opts.default, sol),
            launched_drone_count: ValOptionInt::from_pub(&pub_opts.launched_drone_count, pub_opts.default, sol),
            drone_bandwidth: ValOptionInt::from_pub(&pub_opts.drone_bandwidth, pub_opts.default, sol),
            unlaunchable_drone_slot: ValOptionInt::from_pub(&pub_opts.unlaunchable_drone_slot, pub_opts.default, sol),
            unlaunchable_drone_bandwidth: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_drone_bandwidth,
                pub_opts.default,
                sol,
            ),
            drone_group: ValOptionInt::from_pub(&pub_opts.drone_group, pub_opts.default, sol),
            // Fighters
            fighter_bay_volume: ValOptionInt::from_pub(&pub_opts.fighter_bay_volume, pub_opts.default, sol),
            launched_fighter_count: ValOptionInt::from_pub(&pub_opts.launched_fighter_count, pub_opts.default, sol),
            launched_light_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_light_fighter_count,
                pub_opts.default,
                sol,
            ),
            launched_heavy_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_heavy_fighter_count,
                pub_opts.default,
                sol,
            ),
            launched_support_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_support_fighter_count,
                pub_opts.default,
                sol,
            ),
            launched_st_light_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_st_light_fighter_count,
                pub_opts.default,
                sol,
            ),
            launched_st_heavy_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_st_heavy_fighter_count,
                pub_opts.default,
                sol,
            ),
            launched_st_support_fighter_count: ValOptionInt::from_pub(
                &pub_opts.launched_st_support_fighter_count,
                pub_opts.default,
                sol,
            ),
            unlaunchable_fighter: ValOptionInt::from_pub(&pub_opts.unlaunchable_fighter, pub_opts.default, sol),
            unlaunchable_light_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_light_fighter,
                pub_opts.default,
                sol,
            ),
            unlaunchable_heavy_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_heavy_fighter,
                pub_opts.default,
                sol,
            ),
            unlaunchable_support_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_support_fighter,
                pub_opts.default,
                sol,
            ),
            unlaunchable_st_light_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_st_light_fighter,
                pub_opts.default,
                sol,
            ),
            unlaunchable_st_heavy_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_st_heavy_fighter,
                pub_opts.default,
                sol,
            ),
            unlaunchable_st_support_fighter: ValOptionInt::from_pub(
                &pub_opts.unlaunchable_st_support_fighter,
                pub_opts.default,
                sol,
            ),
            fighter_squad_size: ValOptionInt::from_pub(&pub_opts.fighter_squad_size, pub_opts.default, sol),
            // Projection, destination side
            activation_blocked: ValOptionInt::from_pub(&pub_opts.activation_blocked, pub_opts.default, sol),
            effect_stopper: ValOptionInt::from_pub(&pub_opts.effect_stopper, pub_opts.default, sol),
            cloaking_blocked: ValOptionInt::from_pub(&pub_opts.cloaking_blocked, pub_opts.default, sol),
            // Projection, source side
            projectee_filter: ValOptionInt::from_pub(&pub_opts.projectee_filter, pub_opts.default, sol),
            assist_immunity: ValOptionInt::from_pub(&pub_opts.assist_immunity, pub_opts.default, sol),
            offense_immunity: ValOptionInt::from_pub(&pub_opts.offense_immunity, pub_opts.default, sol),
            resist_immunity: ValOptionInt::from_pub(&pub_opts.resist_immunity, pub_opts.default, sol),
            // Sec zone
            sec_zone_fitted: ValOptionInt::from_pub(&pub_opts.sec_zone_fitted, pub_opts.default, sol),
            sec_zone_online: ValOptionInt::from_pub(&pub_opts.sec_zone_online, pub_opts.default, sol),
            sec_zone_active: ValOptionInt::from_pub(&pub_opts.sec_zone_active, pub_opts.default, sol),
            sec_zone_unonlineable: ValOptionInt::from_pub(&pub_opts.sec_zone_unonlineable, pub_opts.default, sol),
            sec_zone_unactivable: ValOptionInt::from_pub(&pub_opts.sec_zone_unactivable, pub_opts.default, sol),
            sec_zone_effect: ValOptionInt::from_pub(&pub_opts.sec_zone_effect, pub_opts.default, sol),
        }
    }
}

pub(in crate::svc::vast) enum ValOptionInt {
    Enabled(ValOptionEnabledInt),
    Disabled,
}
impl ValOptionInt {
    fn from_pub(pub_opt: &DefOptionExt<ValEnabled>, default: bool, sol: &SolarSystem) -> Self {
        match pub_opt {
            DefOptionExt::Default => match default {
                true => Self::Enabled(ValOptionEnabledInt { kfs: RSet::new() }),
                false => Self::Disabled,
            },
            DefOptionExt::Disabled => Self::Disabled,
            DefOptionExt::Enabled => Self::Enabled(ValOptionEnabledInt { kfs: RSet::new() }),
            DefOptionExt::EnabledExtended(pub_opt) => Self::Enabled(ValOptionEnabledInt {
                kfs: pub_opt
                    .kfs
                    .iter()
                    .filter_map(|item_id| sol.u_data.items.int_id_by_ext_id(item_id))
                    .unique()
                    .collect(),
            }),
        }
    }
}

pub(in crate::svc::vast) struct ValOptionEnabledInt {
    pub(in crate::svc::vast) kfs: RSet<UItemId>,
}
