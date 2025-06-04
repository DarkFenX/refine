use std::collections::HashMap;

use crate::info::valid::details::{
    HValActivationBlockedFail, HValCapitalModFail, HValChargeGroupFail, HValChargeSizeFail, HValChargeVolumeFail,
    HValDroneGroupFail, HValEffectImmunityFail, HValEffectStopperFail, HValFighterSquadSizeFail, HValItemKindFail,
    HValItemVsShipKindFail, HValMaxGroupFail, HValMaxTypeFail, HValModuleStateFail, HValNotLoadedItemFail,
    HValOverloadSkillFail, HValResFail, HValRigSizeFail, HValSecZoneFail, HValShipLimitFail, HValShipStanceFail,
    HValSlotCountFail, HValSlotIndexFail, HValSrqFail, HValUnusableResFail, HValUnusableSlotFail,
};

// Sol-specific
#[derive(serde::Serialize)]
pub(crate) struct HSolValResultDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HSolValDetails::is_empty")]
    details: HSolValDetails,
}
impl From<&rc::val::ValResultSol> for HSolValResultDetailed {
    fn from(core_val_result: &rc::val::ValResultSol) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: core_val_result.into(),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HSolValDetails {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    fits: HashMap<rc::FitId, HValFitInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_loaded_item: Option<HValNotLoadedItemFail>,
}
impl HSolValDetails {
    fn is_empty(&self) -> bool {
        self.fits.is_empty() && self.not_loaded_item.is_none()
    }
}
impl From<&rc::val::ValResultSol> for HSolValDetails {
    fn from(core_val_result: &rc::val::ValResultSol) -> Self {
        Self {
            fits: core_val_result
                .fits
                .iter()
                .map(|(&fit_id, core_fit_val_result)| (fit_id, core_fit_val_result.into()))
                .collect(),
            not_loaded_item: conv(&core_val_result.not_loaded_item),
        }
    }
}

// Fit-specific
#[derive(serde::Serialize)]
pub(crate) struct HFitValResultDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HValFitInfo::is_empty")]
    details: HValFitInfo,
}
impl From<&rc::val::ValResultFit> for HFitValResultDetailed {
    fn from(core_val_result: &rc::val::ValResultFit) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: core_val_result.into(),
        }
    }
}

// Shared
#[serde_with::serde_as]
#[derive(serde::Serialize)]
struct HValFitInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    powergrid: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_bay_volume: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_bandwidth: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fighter_bay_volume: Option<HValResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subsystem_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_drone_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_support_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_light_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_heavy_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_support_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_light_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_heavy_fighter_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turret_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launcher_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mid_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_slot_count: Option<HValSlotCountFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    implant_slot_index: Option<HValSlotIndexFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    booster_slot_index: Option<HValSlotIndexFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subsystem_slot_index: Option<HValSlotIndexFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_limit: Option<HValShipLimitFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_fitted: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_online: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_group_active: Option<HValMaxGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_size: Option<HValRigSizeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skill_reqs: Option<HValSrqFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_group: Option<HValChargeGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_size: Option<HValChargeSizeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_volume: Option<HValChargeVolumeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capital_module: Option<HValCapitalModFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_loaded_item: Option<HValNotLoadedItemFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_state: Option<HValModuleStateFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_kind: Option<HValItemKindFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_group: Option<HValDroneGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fighter_squad_size: Option<HValFighterSquadSizeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_drone_slot: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_drone_bandwidth: Option<HValUnusableResFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_support_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_light_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_heavy_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_standup_support_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_standup_light_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unlaunchable_standup_heavy_fighter: Option<HValUnusableSlotFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_stance: Option<HValShipStanceFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overload_skill: Option<HValOverloadSkillFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_type_fitted: Option<HValMaxTypeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_fitted: Option<HValSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_online: Option<HValSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_active: Option<HValSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_unonlineable: Option<HValSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_zone_unactivable: Option<HValSecZoneFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_blocked: Option<HValActivationBlockedFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_vs_ship_kind: Option<HValItemVsShipKindFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect_stopper: Option<HValEffectStopperFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assist_immunity: Option<HValEffectImmunityFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offense_immunity: Option<HValEffectImmunityFail>,
}
impl HValFitInfo {
    fn is_empty(&self) -> bool {
        self.cpu.is_none()
            && self.powergrid.is_none()
            && self.calibration.is_none()
            && self.drone_bay_volume.is_none()
            && self.drone_bandwidth.is_none()
            && self.fighter_bay_volume.is_none()
            && self.rig_slot_count.is_none()
            && self.service_slot_count.is_none()
            && self.subsystem_slot_count.is_none()
            && self.launched_drone_count.is_none()
            && self.launched_fighter_count.is_none()
            && self.launched_support_fighter_count.is_none()
            && self.launched_light_fighter_count.is_none()
            && self.launched_heavy_fighter_count.is_none()
            && self.launched_standup_support_fighter_count.is_none()
            && self.launched_standup_light_fighter_count.is_none()
            && self.launched_standup_heavy_fighter_count.is_none()
            && self.turret_slot_count.is_none()
            && self.launcher_slot_count.is_none()
            && self.high_slot_count.is_none()
            && self.mid_slot_count.is_none()
            && self.low_slot_count.is_none()
            && self.implant_slot_index.is_none()
            && self.booster_slot_index.is_none()
            && self.subsystem_slot_index.is_none()
            && self.ship_limit.is_none()
            && self.max_group_fitted.is_none()
            && self.max_group_online.is_none()
            && self.max_group_active.is_none()
            && self.rig_size.is_none()
            && self.skill_reqs.is_none()
            && self.charge_group.is_none()
            && self.charge_size.is_none()
            && self.charge_volume.is_none()
            && self.capital_module.is_none()
            && self.not_loaded_item.is_none()
            && self.module_state.is_none()
            && self.item_kind.is_none()
            && self.drone_group.is_none()
            && self.fighter_squad_size.is_none()
            && self.unlaunchable_drone_slot.is_none()
            && self.unlaunchable_drone_bandwidth.is_none()
            && self.unlaunchable_fighter.is_none()
            && self.unlaunchable_support_fighter.is_none()
            && self.unlaunchable_light_fighter.is_none()
            && self.unlaunchable_heavy_fighter.is_none()
            && self.unlaunchable_standup_support_fighter.is_none()
            && self.unlaunchable_standup_light_fighter.is_none()
            && self.unlaunchable_standup_heavy_fighter.is_none()
            && self.ship_stance.is_none()
            && self.overload_skill.is_none()
            && self.max_type_fitted.is_none()
            && self.sec_zone_fitted.is_none()
            && self.sec_zone_online.is_none()
            && self.sec_zone_active.is_none()
            && self.sec_zone_unonlineable.is_none()
            && self.sec_zone_unactivable.is_none()
            && self.activation_blocked.is_none()
            && self.item_vs_ship_kind.is_none()
            && self.effect_stopper.is_none()
            && self.assist_immunity.is_none()
            && self.offense_immunity.is_none()
    }
}
impl From<&rc::val::ValResultFit> for HValFitInfo {
    fn from(core_val_result: &rc::val::ValResultFit) -> Self {
        Self {
            cpu: conv(&core_val_result.cpu),
            powergrid: conv(&core_val_result.powergrid),
            calibration: conv(&core_val_result.calibration),
            drone_bay_volume: conv(&core_val_result.drone_bay_volume),
            drone_bandwidth: conv(&core_val_result.drone_bandwidth),
            fighter_bay_volume: conv(&core_val_result.fighter_bay_volume),
            rig_slot_count: conv(&core_val_result.rig_slot_count),
            service_slot_count: conv(&core_val_result.service_slot_count),
            subsystem_slot_count: conv(&core_val_result.subsystem_slot_count),
            launched_drone_count: conv(&core_val_result.launched_drone_count),
            launched_fighter_count: conv(&core_val_result.launched_fighter_count),
            launched_support_fighter_count: conv(&core_val_result.launched_support_fighter_count),
            launched_light_fighter_count: conv(&core_val_result.launched_light_fighter_count),
            launched_heavy_fighter_count: conv(&core_val_result.launched_heavy_fighter_count),
            launched_standup_support_fighter_count: conv(&core_val_result.launched_standup_support_fighter_count),
            launched_standup_light_fighter_count: conv(&core_val_result.launched_standup_light_fighter_count),
            launched_standup_heavy_fighter_count: conv(&core_val_result.launched_standup_heavy_fighter_count),
            turret_slot_count: conv(&core_val_result.turret_slot_count),
            launcher_slot_count: conv(&core_val_result.launcher_slot_count),
            high_slot_count: conv(&core_val_result.high_slot_count),
            mid_slot_count: conv(&core_val_result.mid_slot_count),
            low_slot_count: conv(&core_val_result.low_slot_count),
            implant_slot_index: conv(&core_val_result.implant_slot_index),
            booster_slot_index: conv(&core_val_result.booster_slot_index),
            subsystem_slot_index: conv(&core_val_result.subsystem_slot_index),
            ship_limit: conv(&core_val_result.ship_limit),
            max_group_fitted: conv(&core_val_result.max_group_fitted),
            max_group_online: conv(&core_val_result.max_group_online),
            max_group_active: conv(&core_val_result.max_group_active),
            rig_size: conv(&core_val_result.rig_size),
            skill_reqs: conv(&core_val_result.skill_reqs),
            charge_group: conv(&core_val_result.charge_group),
            charge_size: conv(&core_val_result.charge_size),
            charge_volume: conv(&core_val_result.charge_volume),
            capital_module: conv(&core_val_result.capital_module),
            not_loaded_item: conv(&core_val_result.not_loaded_item),
            module_state: conv(&core_val_result.module_state),
            item_kind: conv(&core_val_result.item_kind),
            drone_group: conv(&core_val_result.drone_group),
            fighter_squad_size: conv(&core_val_result.fighter_squad_size),
            unlaunchable_drone_slot: conv(&core_val_result.unlaunchable_drone_slot),
            unlaunchable_drone_bandwidth: conv(&core_val_result.unlaunchable_drone_bandwidth),
            unlaunchable_fighter: conv(&core_val_result.unlaunchable_fighter),
            unlaunchable_support_fighter: conv(&core_val_result.unlaunchable_support_fighter),
            unlaunchable_light_fighter: conv(&core_val_result.unlaunchable_light_fighter),
            unlaunchable_heavy_fighter: conv(&core_val_result.unlaunchable_heavy_fighter),
            unlaunchable_standup_support_fighter: conv(&core_val_result.unlaunchable_standup_support_fighter),
            unlaunchable_standup_light_fighter: conv(&core_val_result.unlaunchable_standup_light_fighter),
            unlaunchable_standup_heavy_fighter: conv(&core_val_result.unlaunchable_standup_heavy_fighter),
            ship_stance: conv(&core_val_result.ship_stance),
            overload_skill: conv(&core_val_result.overload_skill),
            max_type_fitted: conv(&core_val_result.max_type_fitted),
            sec_zone_fitted: conv(&core_val_result.sec_zone_fitted),
            sec_zone_online: conv(&core_val_result.sec_zone_online),
            sec_zone_active: conv(&core_val_result.sec_zone_active),
            sec_zone_unonlineable: conv(&core_val_result.sec_zone_unonlineable),
            sec_zone_unactivable: conv(&core_val_result.sec_zone_unactivable),
            activation_blocked: conv(&core_val_result.activation_blocked),
            item_vs_ship_kind: conv(&core_val_result.item_vs_ship_kind),
            effect_stopper: conv(&core_val_result.effect_stopper),
            assist_immunity: conv(&core_val_result.assist_immunity),
            offense_immunity: conv(&core_val_result.offense_immunity),
        }
    }
}

fn conv<T, U>(core_val: &Option<T>) -> Option<U>
where
    U: for<'a> From<&'a T>,
{
    core_val.as_ref().map(|v| v.into())
}
