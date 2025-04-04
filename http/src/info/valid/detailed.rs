use crate::info::valid::details::{
    HValActivationBlockedFail, HValCapitalModFail, HValChargeGroupFail, HValChargeSizeFail, HValChargeVolumeFail,
    HValDroneGroupFail, HValFighterSquadSizeFail, HValItemKindFail, HValItemVsShipKindFail, HValMaxGroupFail,
    HValMaxTypeFail, HValModuleStateFail, HValOverloadSkillFail, HValResFail, HValRigSizeFail, HValSecZoneFail,
    HValShipLimitFail, HValShipStanceFail, HValSlotCountFail, HValSlotIndexFail, HValSrqFail, HValUnusableResFail,
    HValUnusableSlotFail,
};

#[derive(serde::Serialize)]
pub(crate) struct HValidInfoDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HValidInfoDetails::is_empty")]
    details: HValidInfoDetails,
}
impl From<&rc::val::ValResult> for HValidInfoDetailed {
    fn from(core_val_result: &rc::val::ValResult) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: core_val_result.into(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
struct HValidInfoDetails {
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
    #[serde(skip_serializing_if = "HValSlotIndexFail::is_empty")]
    implant_slot_index: HValSlotIndexFail,
    #[serde(skip_serializing_if = "HValSlotIndexFail::is_empty")]
    booster_slot_index: HValSlotIndexFail,
    #[serde(skip_serializing_if = "HValSlotIndexFail::is_empty")]
    subsystem_slot_index: HValSlotIndexFail,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship_limit: Option<HValShipLimitFail>,
    #[serde(skip_serializing_if = "HValMaxGroupFail::is_empty")]
    max_group_fitted: HValMaxGroupFail,
    #[serde(skip_serializing_if = "HValMaxGroupFail::is_empty")]
    max_group_online: HValMaxGroupFail,
    #[serde(skip_serializing_if = "HValMaxGroupFail::is_empty")]
    max_group_active: HValMaxGroupFail,
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_size: Option<HValRigSizeFail>,
    #[serde(skip_serializing_if = "HValSrqFail::is_empty")]
    skill_reqs: HValSrqFail,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_group: Option<HValChargeGroupFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_size: Option<HValChargeSizeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_volume: Option<HValChargeVolumeFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capital_module: Option<HValCapitalModFail>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    not_loaded_item: Vec<rc::ItemId>,
    #[serde(skip_serializing_if = "HValModuleStateFail::is_empty")]
    module_state: HValModuleStateFail,
    #[serde(skip_serializing_if = "HValItemKindFail::is_empty")]
    item_kind: HValItemKindFail,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_group: Option<HValDroneGroupFail>,
    #[serde(skip_serializing_if = "HValFighterSquadSizeFail::is_empty")]
    fighter_squad_size: HValFighterSquadSizeFail,
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
    #[serde(skip_serializing_if = "HValMaxTypeFail::is_empty")]
    max_type_fitted: HValMaxTypeFail,
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
}
impl HValidInfoDetails {
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
            && self.implant_slot_index.is_empty()
            && self.booster_slot_index.is_empty()
            && self.subsystem_slot_index.is_empty()
            && self.ship_limit.is_none()
            && self.max_group_fitted.is_empty()
            && self.max_group_online.is_empty()
            && self.max_group_active.is_empty()
            && self.rig_size.is_none()
            && self.skill_reqs.is_empty()
            && self.charge_group.is_none()
            && self.charge_size.is_none()
            && self.charge_volume.is_none()
            && self.capital_module.is_none()
            && self.not_loaded_item.is_empty()
            && self.module_state.is_empty()
            && self.item_kind.is_empty()
            && self.drone_group.is_none()
            && self.fighter_squad_size.is_empty()
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
            && self.max_type_fitted.is_empty()
            && self.sec_zone_fitted.is_none()
            && self.sec_zone_online.is_none()
            && self.sec_zone_active.is_none()
            && self.sec_zone_unonlineable.is_none()
            && self.sec_zone_unactivable.is_none()
            && self.activation_blocked.is_none()
            && self.item_vs_ship_kind.is_none()
    }
}
impl From<&rc::val::ValResult> for HValidInfoDetails {
    fn from(core_val_result: &rc::val::ValResult) -> Self {
        Self {
            cpu: core_val_result.cpu.as_ref().map(|v| v.into()),
            powergrid: core_val_result.powergrid.as_ref().map(|v| v.into()),
            calibration: core_val_result.calibration.as_ref().map(|v| v.into()),
            drone_bay_volume: core_val_result.drone_bay_volume.as_ref().map(|v| v.into()),
            drone_bandwidth: core_val_result.drone_bandwidth.as_ref().map(|v| v.into()),
            fighter_bay_volume: core_val_result.fighter_bay_volume.as_ref().map(|v| v.into()),
            rig_slot_count: core_val_result.rig_slot_count.as_ref().map(|v| v.into()),
            service_slot_count: core_val_result.service_slot_count.as_ref().map(|v| v.into()),
            subsystem_slot_count: core_val_result.subsystem_slot_count.as_ref().map(|v| v.into()),
            launched_drone_count: core_val_result.launched_drone_count.as_ref().map(|v| v.into()),
            launched_fighter_count: core_val_result.launched_fighter_count.as_ref().map(|v| v.into()),
            launched_support_fighter_count: core_val_result
                .launched_support_fighter_count
                .as_ref()
                .map(|v| v.into()),
            launched_light_fighter_count: core_val_result.launched_light_fighter_count.as_ref().map(|v| v.into()),
            launched_heavy_fighter_count: core_val_result.launched_heavy_fighter_count.as_ref().map(|v| v.into()),
            launched_standup_support_fighter_count: core_val_result
                .launched_standup_support_fighter_count
                .as_ref()
                .map(|v| v.into()),
            launched_standup_light_fighter_count: core_val_result
                .launched_standup_light_fighter_count
                .as_ref()
                .map(|v| v.into()),
            launched_standup_heavy_fighter_count: core_val_result
                .launched_standup_heavy_fighter_count
                .as_ref()
                .map(|v| v.into()),
            turret_slot_count: core_val_result.turret_slot_count.as_ref().map(|v| v.into()),
            launcher_slot_count: core_val_result.launcher_slot_count.as_ref().map(|v| v.into()),
            high_slot_count: core_val_result.high_slot_count.as_ref().map(|v| v.into()),
            mid_slot_count: core_val_result.mid_slot_count.as_ref().map(|v| v.into()),
            low_slot_count: core_val_result.low_slot_count.as_ref().map(|v| v.into()),
            implant_slot_index: (&core_val_result.implant_slot_index).into(),
            booster_slot_index: (&core_val_result.booster_slot_index).into(),
            subsystem_slot_index: (&core_val_result.subsystem_slot_index).into(),
            ship_limit: core_val_result.ship_limit.as_ref().map(|v| v.into()),
            max_group_fitted: (&core_val_result.max_group_fitted).into(),
            max_group_online: (&core_val_result.max_group_online).into(),
            max_group_active: (&core_val_result.max_group_active).into(),
            rig_size: core_val_result.rig_size.as_ref().map(|v| v.into()),
            skill_reqs: (&core_val_result.skill_reqs).into(),
            charge_group: core_val_result.charge_group.as_ref().map(|v| v.into()),
            charge_size: core_val_result.charge_size.as_ref().map(|v| v.into()),
            charge_volume: core_val_result.charge_volume.as_ref().map(|v| v.into()),
            capital_module: core_val_result.capital_module.as_ref().map(|v| v.into()),
            not_loaded_item: core_val_result.not_loaded_item.iter().map(|v| v.item_id).collect(),
            module_state: (&core_val_result.module_state).into(),
            item_kind: (&core_val_result.item_kind).into(),
            drone_group: core_val_result.drone_group.as_ref().map(|v| v.into()),
            fighter_squad_size: (&core_val_result.fighter_squad_size).into(),
            unlaunchable_drone_slot: core_val_result.unlaunchable_drone_slot.as_ref().map(|v| v.into()),
            unlaunchable_drone_bandwidth: core_val_result.unlaunchable_drone_bandwidth.as_ref().map(|v| v.into()),
            unlaunchable_fighter: core_val_result.unlaunchable_fighter.as_ref().map(|v| v.into()),
            unlaunchable_support_fighter: core_val_result.unlaunchable_support_fighter.as_ref().map(|v| v.into()),
            unlaunchable_light_fighter: core_val_result.unlaunchable_light_fighter.as_ref().map(|v| v.into()),
            unlaunchable_heavy_fighter: core_val_result.unlaunchable_heavy_fighter.as_ref().map(|v| v.into()),
            unlaunchable_standup_support_fighter: core_val_result
                .unlaunchable_standup_support_fighter
                .as_ref()
                .map(|v| v.into()),
            unlaunchable_standup_light_fighter: core_val_result
                .unlaunchable_standup_light_fighter
                .as_ref()
                .map(|v| v.into()),
            unlaunchable_standup_heavy_fighter: core_val_result
                .unlaunchable_standup_heavy_fighter
                .as_ref()
                .map(|v| v.into()),
            ship_stance: core_val_result.ship_stance.as_ref().map(|v| v.into()),
            overload_skill: core_val_result.overload_skill.as_ref().map(|v| v.into()),
            max_type_fitted: (&core_val_result.max_type_fitted).into(),
            sec_zone_fitted: core_val_result.sec_zone_fitted.as_ref().map(|v| v.into()),
            sec_zone_online: core_val_result.sec_zone_online.as_ref().map(|v| v.into()),
            sec_zone_active: core_val_result.sec_zone_active.as_ref().map(|v| v.into()),
            sec_zone_unonlineable: core_val_result.sec_zone_unonlineable.as_ref().map(|v| v.into()),
            sec_zone_unactivable: core_val_result.sec_zone_unactivable.as_ref().map(|v| v.into()),
            activation_blocked: core_val_result.activation_blocked.as_ref().map(|v| v.into()),
            item_vs_ship_kind: core_val_result.item_vs_ship_kind.as_ref().map(|v| v.into()),
        }
    }
}
