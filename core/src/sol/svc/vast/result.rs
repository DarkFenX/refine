use std::collections::HashMap;

use crate::sol::{
    FitId,
    svc::vast::{
        ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupFail, ValChargeSizeFail, ValChargeVolumeFail,
        ValDroneGroupFail, ValEffectStopperFail, ValFighterSquadSizeFail, ValItemKindFail, ValItemVsShipKindFail,
        ValMaxGroupFail, ValMaxTypeFail, ValModuleStateFail, ValNotLoadedItemFail, ValOverloadSkillFail,
        ValProjImmunityFail, ValResFail, ValRigSizeFail, ValSecZoneFail, ValShipLimitFail, ValShipStanceFail,
        ValSlotCountFail, ValSlotIndexFail, ValSrqFail, ValUnusableResFail, ValUnusableSlotFail,
    },
};

/// Validation result for a solar system.
///
/// Contains per-fit failures, and failures for items not belonging to any fit.
pub struct ValResultSol {
    pub fits: HashMap<FitId, ValResultFit>,
    /// Not loaded stand-alone items.
    pub not_loaded_item: Option<ValNotLoadedItemFail>,
}
impl ValResultSol {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            fits: HashMap::new(),
            not_loaded_item: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        self.fits.is_empty() && self.not_loaded_item.is_none()
    }
}

/// Validation result for a fit.
pub struct ValResultFit {
    // Generic
    pub not_loaded_item: Option<ValNotLoadedItemFail>,
    pub item_kind: Option<ValItemKindFail>,
    pub skill_reqs: Option<ValSrqFail>,
    // Implants/boosters
    pub implant_slot_index: Option<ValSlotIndexFail>,
    pub booster_slot_index: Option<ValSlotIndexFail>,
    // Shared between mod-alike items
    pub cpu: Option<ValResFail>,
    pub powergrid: Option<ValResFail>,
    pub ship_limit: Option<ValShipLimitFail>,
    pub max_group_fitted: Option<ValMaxGroupFail>,
    pub max_group_online: Option<ValMaxGroupFail>,
    pub max_group_active: Option<ValMaxGroupFail>,
    pub max_type_fitted: Option<ValMaxTypeFail>,
    pub item_vs_ship_kind: Option<ValItemVsShipKindFail>,
    // Modules
    pub high_slot_count: Option<ValSlotCountFail>,
    pub mid_slot_count: Option<ValSlotCountFail>,
    pub low_slot_count: Option<ValSlotCountFail>,
    pub turret_slot_count: Option<ValSlotCountFail>,
    pub launcher_slot_count: Option<ValSlotCountFail>,
    pub module_state: Option<ValModuleStateFail>,
    pub capital_module: Option<ValCapitalModFail>,
    pub overload_skill: Option<ValOverloadSkillFail>,
    // Charges
    pub charge_group: Option<ValChargeGroupFail>,
    pub charge_size: Option<ValChargeSizeFail>,
    pub charge_volume: Option<ValChargeVolumeFail>,
    // Rigs
    pub rig_slot_count: Option<ValSlotCountFail>,
    pub calibration: Option<ValResFail>,
    pub rig_size: Option<ValRigSizeFail>,
    // Services
    pub service_slot_count: Option<ValSlotCountFail>,
    // T3 subsystems/stances
    pub subsystem_slot_count: Option<ValSlotCountFail>,
    pub subsystem_slot_index: Option<ValSlotIndexFail>,
    pub ship_stance: Option<ValShipStanceFail>,
    // Drones
    pub drone_bay_volume: Option<ValResFail>,
    pub launched_drone_count: Option<ValSlotCountFail>,
    pub drone_bandwidth: Option<ValResFail>,
    pub unlaunchable_drone_slot: Option<ValUnusableSlotFail>,
    pub unlaunchable_drone_bandwidth: Option<ValUnusableResFail>,
    pub drone_group: Option<ValDroneGroupFail>,
    // Fighters
    pub fighter_bay_volume: Option<ValResFail>,
    pub launched_fighter_count: Option<ValSlotCountFail>,
    pub launched_light_fighter_count: Option<ValSlotCountFail>,
    pub launched_heavy_fighter_count: Option<ValSlotCountFail>,
    pub launched_support_fighter_count: Option<ValSlotCountFail>,
    pub launched_st_light_fighter_count: Option<ValSlotCountFail>,
    pub launched_st_heavy_fighter_count: Option<ValSlotCountFail>,
    pub launched_st_support_fighter_count: Option<ValSlotCountFail>,
    pub unlaunchable_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_light_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_heavy_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_support_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_st_light_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_st_heavy_fighter: Option<ValUnusableSlotFail>,
    pub unlaunchable_st_support_fighter: Option<ValUnusableSlotFail>,
    pub fighter_squad_size: Option<ValFighterSquadSizeFail>,
    // Projection, destination side
    pub activation_blocked: Option<ValActivationBlockedFail>,
    pub effect_stopper: Option<ValEffectStopperFail>,
    // Projection, source side
    pub assist_immunity: Option<ValProjImmunityFail>,
    pub offense_immunity: Option<ValProjImmunityFail>,
    pub resist_immunity: Option<ValProjImmunityFail>,
    // Sec zone
    pub sec_zone_fitted: Option<ValSecZoneFail>,
    pub sec_zone_online: Option<ValSecZoneFail>,
    pub sec_zone_active: Option<ValSecZoneFail>,
    pub sec_zone_unonlineable: Option<ValSecZoneFail>,
    pub sec_zone_unactivable: Option<ValSecZoneFail>,
}
impl ValResultFit {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            // Generic
            not_loaded_item: None,
            item_kind: None,
            skill_reqs: None,
            // Implants/boosters
            implant_slot_index: None,
            booster_slot_index: None,
            // Shared between mod-alike items
            cpu: None,
            powergrid: None,
            ship_limit: None,
            max_group_fitted: None,
            max_group_online: None,
            max_group_active: None,
            max_type_fitted: None,
            item_vs_ship_kind: None,
            // Modules
            high_slot_count: None,
            mid_slot_count: None,
            low_slot_count: None,
            turret_slot_count: None,
            launcher_slot_count: None,
            module_state: None,
            capital_module: None,
            overload_skill: None,
            // Charges
            charge_group: None,
            charge_size: None,
            charge_volume: None,
            // Rigs
            rig_slot_count: None,
            calibration: None,
            rig_size: None,
            // Services
            service_slot_count: None,
            // T3 subsystems/stances
            subsystem_slot_count: None,
            subsystem_slot_index: None,
            ship_stance: None,
            // Drones
            drone_bay_volume: None,
            launched_drone_count: None,
            drone_bandwidth: None,
            unlaunchable_drone_slot: None,
            unlaunchable_drone_bandwidth: None,
            drone_group: None,
            // Fighters
            fighter_bay_volume: None,
            launched_fighter_count: None,
            launched_light_fighter_count: None,
            launched_heavy_fighter_count: None,
            launched_support_fighter_count: None,
            launched_st_light_fighter_count: None,
            launched_st_heavy_fighter_count: None,
            launched_st_support_fighter_count: None,
            unlaunchable_fighter: None,
            unlaunchable_light_fighter: None,
            unlaunchable_heavy_fighter: None,
            unlaunchable_support_fighter: None,
            unlaunchable_st_light_fighter: None,
            unlaunchable_st_heavy_fighter: None,
            unlaunchable_st_support_fighter: None,
            fighter_squad_size: None,
            // Projection, destination side
            activation_blocked: None,
            effect_stopper: None,
            // Projection, source side
            assist_immunity: None,
            offense_immunity: None,
            resist_immunity: None,
            // Sec zone
            sec_zone_fitted: None,
            sec_zone_online: None,
            sec_zone_active: None,
            sec_zone_unonlineable: None,
            sec_zone_unactivable: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        // Generic
        self.not_loaded_item.is_none()
            && self.item_kind.is_none()
            && self.skill_reqs.is_none()
            // Implants/boosters
            && self.implant_slot_index.is_none()
            && self.booster_slot_index.is_none()
            // Shared between mod-alike items
            && self.cpu.is_none()
            && self.powergrid.is_none()
            && self.ship_limit.is_none()
            && self.max_group_fitted.is_none()
            && self.max_group_online.is_none()
            && self.max_group_active.is_none()
            && self.max_type_fitted.is_none()
            && self.item_vs_ship_kind.is_none()
            // Modules
            && self.high_slot_count.is_none()
            && self.mid_slot_count.is_none()
            && self.low_slot_count.is_none()
            && self.turret_slot_count.is_none()
            && self.launcher_slot_count.is_none()
            && self.module_state.is_none()
            && self.capital_module.is_none()
            && self.overload_skill.is_none()
            // Charges
            && self.charge_group.is_none()
            && self.charge_size.is_none()
            && self.charge_volume.is_none()
            // Rigs
            && self.rig_slot_count.is_none()
            && self.calibration.is_none()
            && self.rig_size.is_none()
            // Services
            && self.service_slot_count.is_none()
            // T3 subsystems/stances
            && self.subsystem_slot_count.is_none()
            && self.subsystem_slot_index.is_none()
            && self.ship_stance.is_none()
            // Drones
            && self.drone_bay_volume.is_none()
            && self.launched_drone_count.is_none()
            && self.drone_bandwidth.is_none()
            && self.unlaunchable_drone_slot.is_none()
            && self.unlaunchable_drone_bandwidth.is_none()
            && self.drone_group.is_none()
            // Fighters
            && self.fighter_bay_volume.is_none()
            && self.launched_fighter_count.is_none()
            && self.launched_light_fighter_count.is_none()
            && self.launched_heavy_fighter_count.is_none()
            && self.launched_support_fighter_count.is_none()
            && self.launched_st_light_fighter_count.is_none()
            && self.launched_st_heavy_fighter_count.is_none()
            && self.launched_st_support_fighter_count.is_none()
            && self.unlaunchable_fighter.is_none()
            && self.unlaunchable_light_fighter.is_none()
            && self.unlaunchable_heavy_fighter.is_none()
            && self.unlaunchable_support_fighter.is_none()
            && self.unlaunchable_st_light_fighter.is_none()
            && self.unlaunchable_st_heavy_fighter.is_none()
            && self.unlaunchable_st_support_fighter.is_none()
            && self.fighter_squad_size.is_none()
            // Projection, destination side
            && self.activation_blocked.is_none()
            && self.effect_stopper.is_none()
            // Projection, source side
            && self.assist_immunity.is_none()
            && self.offense_immunity.is_none()
            && self.resist_immunity.is_none()
            // Sec zone
            && self.sec_zone_fitted.is_none()
            && self.sec_zone_online.is_none()
            && self.sec_zone_active.is_none()
            && self.sec_zone_unonlineable.is_none()
            && self.sec_zone_unactivable.is_none()
    }
}
