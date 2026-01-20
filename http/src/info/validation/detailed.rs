use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::info::validation::details::{
    HValActivationBlockedFail, HValCapitalModFail, HValChargeGroupFail, HValChargeParentGroupFail, HValChargeSizeFail,
    HValChargeVolumeFail, HValDroneGroupFail, HValEffectSecZoneFail, HValEffectStopperFail, HValFighterSquadSizeFail,
    HValItemKindFail, HValItemSecZoneFail, HValItemVsShipKindFail, HValMaxGroupFail, HValMaxTypeFail,
    HValModuleStateFail, HValNotLoadedItemFail, HValOverloadSkillFail, HValProjFilterFail, HValProjImmunityFail,
    HValResourceFail, HValRigSizeFail, HValShipLimitFail, HValShipStanceFail, HValSlotCountFail, HValSlotIndexFail,
    HValSrqFail, HValUnusableCapFail, HValUnusableResFail, HValUnusableSlotFail,
};

// Sol-specific
#[derive(Serialize)]
pub(crate) struct HSolValResultDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HSolValDetails::is_empty")]
    details: HSolValDetails,
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSolValDetails {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fits: Vec<(rc::FitId, HValFitInfo)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_loaded_item: Option<HValNotLoadedItemFail>,
}
impl HSolValDetails {
    fn is_empty(&self) -> bool {
        self.fits.is_empty() && self.not_loaded_item.is_none()
    }
}

// Fit-specific
#[derive(Serialize)]
pub(crate) struct HFitValResultDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HValFitInfo::is_empty")]
    details: HValFitInfo,
}

// Shared
#[serde_as]
#[derive(Serialize)]
struct HValFitInfo {
    // Generic
    #[serde(skip_serializing_if = "Option::is_none")]
    not_loaded_item: Option<HValNotLoadedItemFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_kind: Option<HValItemKindFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skill_reqs: Option<HValSrqFail>,
    // Implants/boosters
    #[serde(skip_serializing_if = "Option::is_none")]
    implant_slot_index: Option<HValSlotIndexFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booster_slot_index: Option<HValSlotIndexFail>,
    // Shared between mod-alike items
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    powergrid: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_limit: Option<HValShipLimitFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_fitted: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_online: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_active: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_type_fitted: Option<HValMaxTypeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_vs_ship_kind: Option<HValItemVsShipKindFail>,
    // Modules
    #[serde(skip_serializing_if = "Option::is_none")]
    high_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mid_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turret_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launcher_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_state: Option<HValModuleStateFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capital_module: Option<HValCapitalModFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overload_skill: Option<HValOverloadSkillFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unusable_cap: Option<HValUnusableCapFail>,
    // Charges
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_group: Option<HValChargeGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_parent_group: Option<HValChargeParentGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_size: Option<HValChargeSizeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_volume: Option<HValChargeVolumeFail>,
    // Rigs
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_size: Option<HValRigSizeFail>,
    // Services
    #[serde(skip_serializing_if = "Option::is_none")]
    service_slot_count: Option<HValSlotCountFail>,
    // T3 subsystems/stances
    #[serde(skip_serializing_if = "Option::is_none")]
    subsystem_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subsystem_slot_index: Option<HValSlotIndexFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_stance: Option<HValShipStanceFail>,
    // Drones
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_bay_volume: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_drone_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_bandwidth: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_drone_slot: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_drone_bandwidth: Option<HValUnusableResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_group: Option<HValDroneGroupFail>,
    // Fighters
    #[serde(skip_serializing_if = "Option::is_none")]
    fighter_bay_volume: Option<HValResourceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_light_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_heavy_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_support_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_st_light_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_st_heavy_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_st_support_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_light_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_heavy_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_support_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_st_light_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_st_heavy_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_st_support_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fighter_squad_size: Option<HValFighterSquadSizeFail>,
    // Projection, destination side
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_blocked: Option<HValActivationBlockedFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect_stopper: Option<HValEffectStopperFail>,
    // Projection, source side
    #[serde(skip_serializing_if = "Option::is_none")]
    projectee_filter: Option<HValProjFilterFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assist_immunity: Option<HValProjImmunityFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offense_immunity: Option<HValProjImmunityFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resist_immunity: Option<HValProjImmunityFail>,
    // Sec zone
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_fitted: Option<HValItemSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_online: Option<HValItemSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_active: Option<HValItemSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_unonlineable: Option<HValItemSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_unactivable: Option<HValItemSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_effect: Option<HValEffectSecZoneFail>,
}
impl HValFitInfo {
    fn is_empty(&self) -> bool {
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
            && self.unusable_cap.is_none()
            // Charges
            && self.charge_group.is_none()
            && self.charge_parent_group.is_none()
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
            && self.projectee_filter.is_none()
            && self.assist_immunity.is_none()
            && self.offense_immunity.is_none()
            && self.resist_immunity.is_none()
            // Sec zone
            && self.sec_zone_fitted.is_none()
            && self.sec_zone_online.is_none()
            && self.sec_zone_active.is_none()
            && self.sec_zone_unonlineable.is_none()
            && self.sec_zone_unactivable.is_none()
            && self.sec_zone_effect.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolValResultDetailed {
    pub(super) fn from_core(core_val_result: rc::val::ValResultSol) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: HSolValDetails::from_core(core_val_result),
        }
    }
}

impl HSolValDetails {
    fn from_core(core_val_result: rc::val::ValResultSol) -> Self {
        Self {
            fits: core_val_result
                .fits
                .into_iter()
                .map(|(fit_id, core_fit_val_result)| (fit_id, HValFitInfo::from_core(core_fit_val_result)))
                .collect(),
            not_loaded_item: core_val_result.not_loaded_item.map(HValNotLoadedItemFail::from_core),
        }
    }
}

impl HFitValResultDetailed {
    pub(super) fn from_core(core_val_result: rc::val::ValResultFit) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: HValFitInfo::from_core(core_val_result),
        }
    }
}

impl HValFitInfo {
    fn from_core(core_val_result: rc::val::ValResultFit) -> Self {
        Self {
            // Generic
            not_loaded_item: core_val_result.not_loaded_item.map(HValNotLoadedItemFail::from_core),
            item_kind: core_val_result.item_kind.map(HValItemKindFail::from_core),
            skill_reqs: core_val_result.skill_reqs.map(HValSrqFail::from_core),
            // Implants/boosters
            implant_slot_index: core_val_result.implant_slot_index.map(HValSlotIndexFail::from_core),
            booster_slot_index: core_val_result.booster_slot_index.map(HValSlotIndexFail::from_core),
            // Shared between mod-alike items
            cpu: core_val_result.cpu.map(HValResourceFail::from_core),
            powergrid: core_val_result.powergrid.map(HValResourceFail::from_core),
            ship_limit: core_val_result.ship_limit.map(HValShipLimitFail::from_core),
            max_group_fitted: core_val_result.max_group_fitted.map(HValMaxGroupFail::from_core),
            max_group_online: core_val_result.max_group_online.map(HValMaxGroupFail::from_core),
            max_group_active: core_val_result.max_group_active.map(HValMaxGroupFail::from_core),
            max_type_fitted: core_val_result.max_type_fitted.map(HValMaxTypeFail::from_core),
            item_vs_ship_kind: core_val_result.item_vs_ship_kind.map(HValItemVsShipKindFail::from_core),
            // Modules
            high_slot_count: core_val_result.high_slot_count.map(HValSlotCountFail::from_core),
            mid_slot_count: core_val_result.mid_slot_count.map(HValSlotCountFail::from_core),
            low_slot_count: core_val_result.low_slot_count.map(HValSlotCountFail::from_core),
            turret_slot_count: core_val_result.turret_slot_count.map(HValSlotCountFail::from_core),
            launcher_slot_count: core_val_result.launcher_slot_count.map(HValSlotCountFail::from_core),
            module_state: core_val_result.module_state.map(HValModuleStateFail::from_core),
            capital_module: core_val_result.capital_module.map(HValCapitalModFail::from_core),
            overload_skill: core_val_result.overload_skill.map(HValOverloadSkillFail::from_core),
            unusable_cap: core_val_result.unusable_cap.map(HValUnusableCapFail::from_core),
            // Charges
            charge_group: core_val_result.charge_group.map(HValChargeGroupFail::from_core),
            charge_parent_group: core_val_result
                .charge_parent_group
                .map(HValChargeParentGroupFail::from_core),
            charge_size: core_val_result.charge_size.map(HValChargeSizeFail::from_core),
            charge_volume: core_val_result.charge_volume.map(HValChargeVolumeFail::from_core),
            // Rigs
            rig_slot_count: core_val_result.rig_slot_count.map(HValSlotCountFail::from_core),
            calibration: core_val_result.calibration.map(HValResourceFail::from_core),
            rig_size: core_val_result.rig_size.map(HValRigSizeFail::from_core),
            // Services
            service_slot_count: core_val_result.service_slot_count.map(HValSlotCountFail::from_core),
            // T3 subsystems/stances
            subsystem_slot_count: core_val_result.subsystem_slot_count.map(HValSlotCountFail::from_core),
            subsystem_slot_index: core_val_result.subsystem_slot_index.map(HValSlotIndexFail::from_core),
            ship_stance: core_val_result.ship_stance.map(HValShipStanceFail::from_core),
            // Drones
            drone_bay_volume: core_val_result.drone_bay_volume.map(HValResourceFail::from_core),
            launched_drone_count: core_val_result.launched_drone_count.map(HValSlotCountFail::from_core),
            drone_bandwidth: core_val_result.drone_bandwidth.map(HValResourceFail::from_core),
            unlaunchable_drone_slot: core_val_result
                .unlaunchable_drone_slot
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_drone_bandwidth: core_val_result
                .unlaunchable_drone_bandwidth
                .map(HValUnusableResFail::from_core),
            drone_group: core_val_result.drone_group.map(HValDroneGroupFail::from_core),
            // Fighters
            fighter_bay_volume: core_val_result.fighter_bay_volume.map(HValResourceFail::from_core),
            launched_fighter_count: core_val_result.launched_fighter_count.map(HValSlotCountFail::from_core),
            launched_light_fighter_count: core_val_result
                .launched_light_fighter_count
                .map(HValSlotCountFail::from_core),
            launched_heavy_fighter_count: core_val_result
                .launched_heavy_fighter_count
                .map(HValSlotCountFail::from_core),
            launched_support_fighter_count: core_val_result
                .launched_support_fighter_count
                .map(HValSlotCountFail::from_core),
            launched_st_light_fighter_count: core_val_result
                .launched_st_light_fighter_count
                .map(HValSlotCountFail::from_core),
            launched_st_heavy_fighter_count: core_val_result
                .launched_st_heavy_fighter_count
                .map(HValSlotCountFail::from_core),
            launched_st_support_fighter_count: core_val_result
                .launched_st_support_fighter_count
                .map(HValSlotCountFail::from_core),
            unlaunchable_fighter: core_val_result
                .unlaunchable_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_light_fighter: core_val_result
                .unlaunchable_light_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_heavy_fighter: core_val_result
                .unlaunchable_heavy_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_support_fighter: core_val_result
                .unlaunchable_support_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_st_light_fighter: core_val_result
                .unlaunchable_st_light_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_st_heavy_fighter: core_val_result
                .unlaunchable_st_heavy_fighter
                .map(HValUnusableSlotFail::from_core),
            unlaunchable_st_support_fighter: core_val_result
                .unlaunchable_st_support_fighter
                .map(HValUnusableSlotFail::from_core),
            fighter_squad_size: core_val_result
                .fighter_squad_size
                .map(HValFighterSquadSizeFail::from_core),
            // Projection, destination side
            activation_blocked: core_val_result
                .activation_blocked
                .map(HValActivationBlockedFail::from_core),
            effect_stopper: core_val_result.effect_stopper.map(HValEffectStopperFail::from_core),
            // Projection, source side
            projectee_filter: core_val_result.projectee_filter.map(HValProjFilterFail::from_core),
            assist_immunity: core_val_result.assist_immunity.map(HValProjImmunityFail::from_core),
            offense_immunity: core_val_result.offense_immunity.map(HValProjImmunityFail::from_core),
            resist_immunity: core_val_result.resist_immunity.map(HValProjImmunityFail::from_core),
            // Sec zone
            sec_zone_fitted: core_val_result.sec_zone_fitted.map(HValItemSecZoneFail::from_core),
            sec_zone_online: core_val_result.sec_zone_online.map(HValItemSecZoneFail::from_core),
            sec_zone_active: core_val_result.sec_zone_active.map(HValItemSecZoneFail::from_core),
            sec_zone_unonlineable: core_val_result
                .sec_zone_unonlineable
                .map(HValItemSecZoneFail::from_core),
            sec_zone_unactivable: core_val_result.sec_zone_unactivable.map(HValItemSecZoneFail::from_core),
            sec_zone_effect: core_val_result.sec_zone_effect.map(HValEffectSecZoneFail::from_core),
        }
    }
}
