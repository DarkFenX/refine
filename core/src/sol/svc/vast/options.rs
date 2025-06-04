use itertools::Itertools;

use crate::{
    sol::{FitId, FitKey, ItemId, ItemKey, SolarSystem},
    util::RSet,
};

#[derive(Clone)]
pub struct SolValOptions {
    pub options: ValOptions,
    pub fit_ids: Vec<FitId>,
}
impl SolValOptions {
    pub fn all_enabled() -> Self {
        Self {
            options: ValOptions::all_enabled(),
            fit_ids: Vec::new(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            options: ValOptions::all_disabled(),
            fit_ids: Vec::new(),
        }
    }
}

pub(in crate::sol) struct IntSolValOptions {
    pub(in crate::sol) options: IntValOptions,
    pub(in crate::sol) fit_keys: Vec<FitKey>,
}
impl IntSolValOptions {
    pub(in crate::sol) fn from_pub_sol_options(sol: &SolarSystem, pub_sol_opts: &SolValOptions) -> Self {
        Self {
            options: IntValOptions::from_pub_options(sol, &pub_sol_opts.options),
            fit_keys: pub_sol_opts
                .fit_ids
                .iter()
                .filter_map(|fit_id| sol.uad.fits.key_by_id(fit_id))
                .unique()
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct ValOptions {
    pub cpu: ValOption,
    pub powergrid: ValOption,
    pub calibration: ValOption,
    pub drone_bay_volume: ValOption,
    pub drone_bandwidth: ValOption,
    pub fighter_bay_volume: ValOption,
    pub rig_slot_count: ValOption,
    pub service_slot_count: ValOption,
    pub subsystem_slot_count: ValOption,
    pub launched_drone_count: ValOption,
    pub launched_fighter_count: ValOption,
    pub launched_support_fighter_count: ValOption,
    pub launched_light_fighter_count: ValOption,
    pub launched_heavy_fighter_count: ValOption,
    pub launched_standup_support_fighter_count: ValOption,
    pub launched_standup_light_fighter_count: ValOption,
    pub launched_standup_heavy_fighter_count: ValOption,
    pub turret_slot_count: ValOption,
    pub launcher_slot_count: ValOption,
    pub high_slot_count: ValOption,
    pub mid_slot_count: ValOption,
    pub low_slot_count: ValOption,
    pub implant_slot_index: ValOption,
    pub booster_slot_index: ValOption,
    pub subsystem_slot_index: ValOption,
    pub ship_limit: ValOption,
    pub max_group_fitted: ValOption,
    pub max_group_online: ValOption,
    pub max_group_active: ValOption,
    pub rig_size: ValOption,
    pub skill_reqs: ValOption,
    pub charge_group: ValOption,
    pub charge_size: ValOption,
    pub charge_volume: ValOption,
    pub capital_module: ValOption,
    pub not_loaded_item: ValOption,
    pub module_state: ValOption,
    pub item_kind: ValOption,
    pub drone_group: ValOption,
    pub fighter_squad_size: ValOption,
    pub unlaunchable_drone_slot: ValOption,
    pub unlaunchable_drone_bandwidth: ValOption,
    pub unlaunchable_fighter: ValOption,
    pub unlaunchable_support_fighter: ValOption,
    pub unlaunchable_light_fighter: ValOption,
    pub unlaunchable_heavy_fighter: ValOption,
    pub unlaunchable_standup_support_fighter: ValOption,
    pub unlaunchable_standup_light_fighter: ValOption,
    pub unlaunchable_standup_heavy_fighter: ValOption,
    pub ship_stance: ValOption,
    pub overload_skill: ValOption,
    pub max_type_fitted: ValOption,
    pub sec_zone_fitted: ValOption,
    pub sec_zone_online: ValOption,
    pub sec_zone_active: ValOption,
    pub sec_zone_unonlineable: ValOption,
    pub sec_zone_unactivable: ValOption,
    pub activation_blocked: ValOption,
    pub item_vs_ship_kind: ValOption,
    pub effect_stopper: ValOption,
    pub assist_immunity: ValOption,
    pub offense_immunity: ValOption,
}
impl ValOptions {
    pub fn all_enabled() -> Self {
        Self {
            cpu: ValOption::new_enabled(),
            powergrid: ValOption::new_enabled(),
            calibration: ValOption::new_enabled(),
            drone_bay_volume: ValOption::new_enabled(),
            drone_bandwidth: ValOption::new_enabled(),
            fighter_bay_volume: ValOption::new_enabled(),
            rig_slot_count: ValOption::new_enabled(),
            service_slot_count: ValOption::new_enabled(),
            subsystem_slot_count: ValOption::new_enabled(),
            launched_drone_count: ValOption::new_enabled(),
            launched_fighter_count: ValOption::new_enabled(),
            launched_support_fighter_count: ValOption::new_enabled(),
            launched_light_fighter_count: ValOption::new_enabled(),
            launched_heavy_fighter_count: ValOption::new_enabled(),
            launched_standup_support_fighter_count: ValOption::new_enabled(),
            launched_standup_light_fighter_count: ValOption::new_enabled(),
            launched_standup_heavy_fighter_count: ValOption::new_enabled(),
            turret_slot_count: ValOption::new_enabled(),
            launcher_slot_count: ValOption::new_enabled(),
            high_slot_count: ValOption::new_enabled(),
            mid_slot_count: ValOption::new_enabled(),
            low_slot_count: ValOption::new_enabled(),
            implant_slot_index: ValOption::new_enabled(),
            booster_slot_index: ValOption::new_enabled(),
            subsystem_slot_index: ValOption::new_enabled(),
            ship_limit: ValOption::new_enabled(),
            max_group_fitted: ValOption::new_enabled(),
            max_group_online: ValOption::new_enabled(),
            max_group_active: ValOption::new_enabled(),
            rig_size: ValOption::new_enabled(),
            skill_reqs: ValOption::new_enabled(),
            charge_group: ValOption::new_enabled(),
            charge_size: ValOption::new_enabled(),
            charge_volume: ValOption::new_enabled(),
            capital_module: ValOption::new_enabled(),
            not_loaded_item: ValOption::new_enabled(),
            module_state: ValOption::new_enabled(),
            item_kind: ValOption::new_enabled(),
            drone_group: ValOption::new_enabled(),
            fighter_squad_size: ValOption::new_enabled(),
            unlaunchable_drone_slot: ValOption::new_enabled(),
            unlaunchable_drone_bandwidth: ValOption::new_enabled(),
            unlaunchable_fighter: ValOption::new_enabled(),
            unlaunchable_support_fighter: ValOption::new_enabled(),
            unlaunchable_light_fighter: ValOption::new_enabled(),
            unlaunchable_heavy_fighter: ValOption::new_enabled(),
            unlaunchable_standup_support_fighter: ValOption::new_enabled(),
            unlaunchable_standup_light_fighter: ValOption::new_enabled(),
            unlaunchable_standup_heavy_fighter: ValOption::new_enabled(),
            ship_stance: ValOption::new_enabled(),
            overload_skill: ValOption::new_enabled(),
            max_type_fitted: ValOption::new_enabled(),
            sec_zone_fitted: ValOption::new_enabled(),
            sec_zone_online: ValOption::new_enabled(),
            sec_zone_active: ValOption::new_enabled(),
            sec_zone_unonlineable: ValOption::new_enabled(),
            sec_zone_unactivable: ValOption::new_enabled(),
            activation_blocked: ValOption::new_enabled(),
            item_vs_ship_kind: ValOption::new_enabled(),
            effect_stopper: ValOption::new_enabled(),
            assist_immunity: ValOption::new_enabled(),
            offense_immunity: ValOption::new_enabled(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            cpu: ValOption::new_disabled(),
            powergrid: ValOption::new_disabled(),
            calibration: ValOption::new_disabled(),
            drone_bay_volume: ValOption::new_disabled(),
            drone_bandwidth: ValOption::new_disabled(),
            fighter_bay_volume: ValOption::new_disabled(),
            rig_slot_count: ValOption::new_disabled(),
            service_slot_count: ValOption::new_disabled(),
            subsystem_slot_count: ValOption::new_disabled(),
            launched_drone_count: ValOption::new_disabled(),
            launched_fighter_count: ValOption::new_disabled(),
            launched_support_fighter_count: ValOption::new_disabled(),
            launched_light_fighter_count: ValOption::new_disabled(),
            launched_heavy_fighter_count: ValOption::new_disabled(),
            launched_standup_support_fighter_count: ValOption::new_disabled(),
            launched_standup_light_fighter_count: ValOption::new_disabled(),
            launched_standup_heavy_fighter_count: ValOption::new_disabled(),
            turret_slot_count: ValOption::new_disabled(),
            launcher_slot_count: ValOption::new_disabled(),
            high_slot_count: ValOption::new_disabled(),
            mid_slot_count: ValOption::new_disabled(),
            low_slot_count: ValOption::new_disabled(),
            implant_slot_index: ValOption::new_disabled(),
            booster_slot_index: ValOption::new_disabled(),
            subsystem_slot_index: ValOption::new_disabled(),
            ship_limit: ValOption::new_disabled(),
            max_group_fitted: ValOption::new_disabled(),
            max_group_online: ValOption::new_disabled(),
            max_group_active: ValOption::new_disabled(),
            rig_size: ValOption::new_disabled(),
            skill_reqs: ValOption::new_disabled(),
            charge_group: ValOption::new_disabled(),
            charge_size: ValOption::new_disabled(),
            charge_volume: ValOption::new_disabled(),
            capital_module: ValOption::new_disabled(),
            not_loaded_item: ValOption::new_disabled(),
            module_state: ValOption::new_disabled(),
            item_kind: ValOption::new_disabled(),
            drone_group: ValOption::new_disabled(),
            fighter_squad_size: ValOption::new_disabled(),
            unlaunchable_drone_slot: ValOption::new_disabled(),
            unlaunchable_drone_bandwidth: ValOption::new_disabled(),
            unlaunchable_fighter: ValOption::new_disabled(),
            unlaunchable_support_fighter: ValOption::new_disabled(),
            unlaunchable_light_fighter: ValOption::new_disabled(),
            unlaunchable_heavy_fighter: ValOption::new_disabled(),
            unlaunchable_standup_support_fighter: ValOption::new_disabled(),
            unlaunchable_standup_light_fighter: ValOption::new_disabled(),
            unlaunchable_standup_heavy_fighter: ValOption::new_disabled(),
            ship_stance: ValOption::new_disabled(),
            overload_skill: ValOption::new_disabled(),
            max_type_fitted: ValOption::new_disabled(),
            sec_zone_fitted: ValOption::new_disabled(),
            sec_zone_online: ValOption::new_disabled(),
            sec_zone_active: ValOption::new_disabled(),
            sec_zone_unonlineable: ValOption::new_disabled(),
            sec_zone_unactivable: ValOption::new_disabled(),
            activation_blocked: ValOption::new_disabled(),
            item_vs_ship_kind: ValOption::new_disabled(),
            effect_stopper: ValOption::new_disabled(),
            assist_immunity: ValOption::new_disabled(),
            offense_immunity: ValOption::new_disabled(),
        }
    }
}

pub(in crate::sol) struct IntValOptions {
    pub(in crate::sol::svc::vast) cpu: IntValOption,
    pub(in crate::sol::svc::vast) powergrid: IntValOption,
    pub(in crate::sol::svc::vast) calibration: IntValOption,
    pub(in crate::sol::svc::vast) drone_bay_volume: IntValOption,
    pub(in crate::sol::svc::vast) drone_bandwidth: IntValOption,
    pub(in crate::sol::svc::vast) fighter_bay_volume: IntValOption,
    pub(in crate::sol::svc::vast) rig_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) service_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) subsystem_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_drone_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_support_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_light_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_heavy_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_standup_support_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_standup_light_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) launched_standup_heavy_fighter_count: IntValOption,
    pub(in crate::sol::svc::vast) turret_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) launcher_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) high_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) mid_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) low_slot_count: IntValOption,
    pub(in crate::sol::svc::vast) implant_slot_index: IntValOption,
    pub(in crate::sol::svc::vast) booster_slot_index: IntValOption,
    pub(in crate::sol::svc::vast) subsystem_slot_index: IntValOption,
    pub(in crate::sol::svc::vast) ship_limit: IntValOption,
    pub(in crate::sol::svc::vast) max_group_fitted: IntValOption,
    pub(in crate::sol::svc::vast) max_group_online: IntValOption,
    pub(in crate::sol::svc::vast) max_group_active: IntValOption,
    pub(in crate::sol::svc::vast) rig_size: IntValOption,
    pub(in crate::sol::svc::vast) skill_reqs: IntValOption,
    pub(in crate::sol::svc::vast) charge_group: IntValOption,
    pub(in crate::sol::svc::vast) charge_size: IntValOption,
    pub(in crate::sol::svc::vast) charge_volume: IntValOption,
    pub(in crate::sol::svc::vast) capital_module: IntValOption,
    pub(in crate::sol::svc::vast) not_loaded_item: IntValOption,
    pub(in crate::sol::svc::vast) module_state: IntValOption,
    pub(in crate::sol::svc::vast) item_kind: IntValOption,
    pub(in crate::sol::svc::vast) drone_group: IntValOption,
    pub(in crate::sol::svc::vast) fighter_squad_size: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_drone_slot: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_drone_bandwidth: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_support_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_light_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_heavy_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_standup_support_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_standup_light_fighter: IntValOption,
    pub(in crate::sol::svc::vast) unlaunchable_standup_heavy_fighter: IntValOption,
    pub(in crate::sol::svc::vast) ship_stance: IntValOption,
    pub(in crate::sol::svc::vast) overload_skill: IntValOption,
    pub(in crate::sol::svc::vast) max_type_fitted: IntValOption,
    pub(in crate::sol::svc::vast) sec_zone_fitted: IntValOption,
    pub(in crate::sol::svc::vast) sec_zone_online: IntValOption,
    pub(in crate::sol::svc::vast) sec_zone_active: IntValOption,
    pub(in crate::sol::svc::vast) sec_zone_unonlineable: IntValOption,
    pub(in crate::sol::svc::vast) sec_zone_unactivable: IntValOption,
    pub(in crate::sol::svc::vast) activation_blocked: IntValOption,
    pub(in crate::sol::svc::vast) item_vs_ship_kind: IntValOption,
    pub(in crate::sol::svc::vast) effect_stopper: IntValOption,
    pub(in crate::sol::svc::vast) assist_immunity: IntValOption,
    pub(in crate::sol::svc::vast) offense_immunity: IntValOption,
}
impl IntValOptions {
    pub(in crate::sol) fn from_pub_options(sol: &SolarSystem, pub_opts: &ValOptions) -> Self {
        Self {
            cpu: IntValOption::from_pub_option(sol, &pub_opts.cpu),
            powergrid: IntValOption::from_pub_option(sol, &pub_opts.powergrid),
            calibration: IntValOption::from_pub_option(sol, &pub_opts.calibration),
            drone_bay_volume: IntValOption::from_pub_option(sol, &pub_opts.drone_bay_volume),
            drone_bandwidth: IntValOption::from_pub_option(sol, &pub_opts.drone_bandwidth),
            fighter_bay_volume: IntValOption::from_pub_option(sol, &pub_opts.fighter_bay_volume),
            rig_slot_count: IntValOption::from_pub_option(sol, &pub_opts.rig_slot_count),
            service_slot_count: IntValOption::from_pub_option(sol, &pub_opts.service_slot_count),
            subsystem_slot_count: IntValOption::from_pub_option(sol, &pub_opts.subsystem_slot_count),
            launched_drone_count: IntValOption::from_pub_option(sol, &pub_opts.launched_drone_count),
            launched_fighter_count: IntValOption::from_pub_option(sol, &pub_opts.launched_fighter_count),
            launched_support_fighter_count: IntValOption::from_pub_option(
                sol,
                &pub_opts.launched_support_fighter_count,
            ),
            launched_light_fighter_count: IntValOption::from_pub_option(sol, &pub_opts.launched_light_fighter_count),
            launched_heavy_fighter_count: IntValOption::from_pub_option(sol, &pub_opts.launched_heavy_fighter_count),
            launched_standup_support_fighter_count: IntValOption::from_pub_option(
                sol,
                &pub_opts.launched_standup_support_fighter_count,
            ),
            launched_standup_light_fighter_count: IntValOption::from_pub_option(
                sol,
                &pub_opts.launched_standup_light_fighter_count,
            ),
            launched_standup_heavy_fighter_count: IntValOption::from_pub_option(
                sol,
                &pub_opts.launched_standup_heavy_fighter_count,
            ),
            turret_slot_count: IntValOption::from_pub_option(sol, &pub_opts.turret_slot_count),
            launcher_slot_count: IntValOption::from_pub_option(sol, &pub_opts.launcher_slot_count),
            high_slot_count: IntValOption::from_pub_option(sol, &pub_opts.high_slot_count),
            mid_slot_count: IntValOption::from_pub_option(sol, &pub_opts.mid_slot_count),
            low_slot_count: IntValOption::from_pub_option(sol, &pub_opts.low_slot_count),
            implant_slot_index: IntValOption::from_pub_option(sol, &pub_opts.implant_slot_index),
            booster_slot_index: IntValOption::from_pub_option(sol, &pub_opts.booster_slot_index),
            subsystem_slot_index: IntValOption::from_pub_option(sol, &pub_opts.subsystem_slot_index),
            ship_limit: IntValOption::from_pub_option(sol, &pub_opts.ship_limit),
            max_group_fitted: IntValOption::from_pub_option(sol, &pub_opts.max_group_fitted),
            max_group_online: IntValOption::from_pub_option(sol, &pub_opts.max_group_online),
            max_group_active: IntValOption::from_pub_option(sol, &pub_opts.max_group_active),
            rig_size: IntValOption::from_pub_option(sol, &pub_opts.rig_size),
            skill_reqs: IntValOption::from_pub_option(sol, &pub_opts.skill_reqs),
            charge_group: IntValOption::from_pub_option(sol, &pub_opts.charge_group),
            charge_size: IntValOption::from_pub_option(sol, &pub_opts.charge_size),
            charge_volume: IntValOption::from_pub_option(sol, &pub_opts.charge_volume),
            capital_module: IntValOption::from_pub_option(sol, &pub_opts.capital_module),
            not_loaded_item: IntValOption::from_pub_option(sol, &pub_opts.not_loaded_item),
            module_state: IntValOption::from_pub_option(sol, &pub_opts.module_state),
            item_kind: IntValOption::from_pub_option(sol, &pub_opts.item_kind),
            drone_group: IntValOption::from_pub_option(sol, &pub_opts.drone_group),
            fighter_squad_size: IntValOption::from_pub_option(sol, &pub_opts.fighter_squad_size),
            unlaunchable_drone_slot: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_drone_slot),
            unlaunchable_drone_bandwidth: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_drone_bandwidth),
            unlaunchable_fighter: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_fighter),
            unlaunchable_support_fighter: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_support_fighter),
            unlaunchable_light_fighter: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_light_fighter),
            unlaunchable_heavy_fighter: IntValOption::from_pub_option(sol, &pub_opts.unlaunchable_heavy_fighter),
            unlaunchable_standup_support_fighter: IntValOption::from_pub_option(
                sol,
                &pub_opts.unlaunchable_standup_support_fighter,
            ),
            unlaunchable_standup_light_fighter: IntValOption::from_pub_option(
                sol,
                &pub_opts.unlaunchable_standup_light_fighter,
            ),
            unlaunchable_standup_heavy_fighter: IntValOption::from_pub_option(
                sol,
                &pub_opts.unlaunchable_standup_heavy_fighter,
            ),
            ship_stance: IntValOption::from_pub_option(sol, &pub_opts.ship_stance),
            overload_skill: IntValOption::from_pub_option(sol, &pub_opts.overload_skill),
            max_type_fitted: IntValOption::from_pub_option(sol, &pub_opts.max_type_fitted),
            sec_zone_fitted: IntValOption::from_pub_option(sol, &pub_opts.sec_zone_fitted),
            sec_zone_online: IntValOption::from_pub_option(sol, &pub_opts.sec_zone_online),
            sec_zone_active: IntValOption::from_pub_option(sol, &pub_opts.sec_zone_active),
            sec_zone_unonlineable: IntValOption::from_pub_option(sol, &pub_opts.sec_zone_unonlineable),
            sec_zone_unactivable: IntValOption::from_pub_option(sol, &pub_opts.sec_zone_unactivable),
            activation_blocked: IntValOption::from_pub_option(sol, &pub_opts.activation_blocked),
            item_vs_ship_kind: IntValOption::from_pub_option(sol, &pub_opts.item_vs_ship_kind),
            effect_stopper: IntValOption::from_pub_option(sol, &pub_opts.effect_stopper),
            assist_immunity: IntValOption::from_pub_option(sol, &pub_opts.assist_immunity),
            offense_immunity: IntValOption::from_pub_option(sol, &pub_opts.offense_immunity),
        }
    }
}

#[derive(Clone)]
pub struct ValOption {
    pub enabled: bool,
    pub kfs: Vec<ItemId>,
}
impl ValOption {
    pub fn new_enabled() -> Self {
        Self {
            enabled: true,
            kfs: Vec::new(),
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            kfs: Vec::new(),
        }
    }
}

pub(in crate::sol::svc::vast) struct IntValOption {
    pub(in crate::sol::svc::vast) enabled: bool,
    pub(in crate::sol::svc::vast) kfs: RSet<ItemKey>,
}
impl IntValOption {
    fn from_pub_option(sol: &SolarSystem, pub_opt: &ValOption) -> Self {
        Self {
            enabled: pub_opt.enabled,
            kfs: pub_opt
                .kfs
                .iter()
                .filter_map(|item_id| sol.uad.items.key_by_id(item_id))
                .unique()
                .collect(),
        }
    }
}
