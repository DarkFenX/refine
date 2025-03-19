use crate::sol::svc::vast::{
    SolValCapitalModFail, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail, SolValDroneGroupFail,
    SolValFighterCountFail, SolValItemKindFail, SolValMaxGroupFail, SolValMaxTypeFail, SolValModuleStateFail,
    SolValNotLoadedItemFail, SolValOverloadSkillFail, SolValResFail, SolValRigSizeFail, SolValSecZoneFail,
    SolValShipLimitFail, SolValShipStanceFail, SolValSlotCountFail, SolValSlotIndexFail, SolValSrqFail,
    SolValUnusableResFail, SolValUnusableSlotFail,
};

pub struct SolValResult {
    pub cpu: Option<SolValResFail>,
    pub powergrid: Option<SolValResFail>,
    pub calibration: Option<SolValResFail>,
    pub drone_bay_volume: Option<SolValResFail>,
    pub drone_bandwidth: Option<SolValResFail>,
    pub fighter_bay_volume: Option<SolValResFail>,
    pub rig_slot_count: Option<SolValSlotCountFail>,
    pub service_slot_count: Option<SolValSlotCountFail>,
    pub subsystem_slot_count: Option<SolValSlotCountFail>,
    pub launched_drone_count: Option<SolValSlotCountFail>,
    pub launched_fighter_count: Option<SolValSlotCountFail>,
    pub launched_support_fighter_count: Option<SolValSlotCountFail>,
    pub launched_light_fighter_count: Option<SolValSlotCountFail>,
    pub launched_heavy_fighter_count: Option<SolValSlotCountFail>,
    pub launched_standup_support_fighter_count: Option<SolValSlotCountFail>,
    pub launched_standup_light_fighter_count: Option<SolValSlotCountFail>,
    pub launched_standup_heavy_fighter_count: Option<SolValSlotCountFail>,
    pub turret_slot_count: Option<SolValSlotCountFail>,
    pub launcher_slot_count: Option<SolValSlotCountFail>,
    pub high_slot_count: Option<SolValSlotCountFail>,
    pub mid_slot_count: Option<SolValSlotCountFail>,
    pub low_slot_count: Option<SolValSlotCountFail>,
    pub implant_slot_index: Vec<SolValSlotIndexFail>,
    pub booster_slot_index: Vec<SolValSlotIndexFail>,
    pub subsystem_slot_index: Vec<SolValSlotIndexFail>,
    pub ship_limit: Option<SolValShipLimitFail>,
    pub max_group_fitted: Vec<SolValMaxGroupFail>,
    pub max_group_online: Vec<SolValMaxGroupFail>,
    pub max_group_active: Vec<SolValMaxGroupFail>,
    pub rig_size: Option<SolValRigSizeFail>,
    pub skill_reqs: Vec<SolValSrqFail>,
    pub charge_group: Vec<SolValChargeGroupFail>,
    pub charge_size: Vec<SolValChargeSizeFail>,
    pub charge_volume: Vec<SolValChargeVolumeFail>,
    pub capital_module: Option<SolValCapitalModFail>,
    pub not_loaded_item: Vec<SolValNotLoadedItemFail>,
    pub module_state: Vec<SolValModuleStateFail>,
    pub item_kind: Vec<SolValItemKindFail>,
    pub drone_group: Option<SolValDroneGroupFail>,
    pub fighter_count: Vec<SolValFighterCountFail>,
    pub unlaunchable_drone_slot: Option<SolValUnusableSlotFail>,
    pub unlaunchable_drone_bandwidth: Option<SolValUnusableResFail>,
    pub unlaunchable_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_support_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_light_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_heavy_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_standup_support_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_standup_light_fighter: Option<SolValUnusableSlotFail>,
    pub unlaunchable_standup_heavy_fighter: Option<SolValUnusableSlotFail>,
    pub ship_stance: Option<SolValShipStanceFail>,
    pub overload_skill: Option<SolValOverloadSkillFail>,
    pub max_type_fitted: Vec<SolValMaxTypeFail>,
    pub sec_zone_fitted: Option<SolValSecZoneFail>,
    pub sec_zone_online: Option<SolValSecZoneFail>,
    pub sec_zone_active: Option<SolValSecZoneFail>,
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            cpu: None,
            powergrid: None,
            calibration: None,
            drone_bay_volume: None,
            drone_bandwidth: None,
            fighter_bay_volume: None,
            rig_slot_count: None,
            service_slot_count: None,
            subsystem_slot_count: None,
            launched_drone_count: None,
            launched_fighter_count: None,
            launched_support_fighter_count: None,
            launched_light_fighter_count: None,
            launched_heavy_fighter_count: None,
            launched_standup_support_fighter_count: None,
            launched_standup_light_fighter_count: None,
            launched_standup_heavy_fighter_count: None,
            turret_slot_count: None,
            launcher_slot_count: None,
            high_slot_count: None,
            mid_slot_count: None,
            low_slot_count: None,
            implant_slot_index: Vec::new(),
            booster_slot_index: Vec::new(),
            subsystem_slot_index: Vec::new(),
            ship_limit: None,
            max_group_fitted: Vec::new(),
            max_group_online: Vec::new(),
            max_group_active: Vec::new(),
            rig_size: None,
            skill_reqs: Vec::new(),
            charge_group: Vec::new(),
            charge_size: Vec::new(),
            charge_volume: Vec::new(),
            capital_module: None,
            not_loaded_item: Vec::new(),
            module_state: Vec::new(),
            item_kind: Vec::new(),
            drone_group: None,
            fighter_count: Vec::new(),
            unlaunchable_drone_slot: None,
            unlaunchable_drone_bandwidth: None,
            unlaunchable_fighter: None,
            unlaunchable_support_fighter: None,
            unlaunchable_light_fighter: None,
            unlaunchable_heavy_fighter: None,
            unlaunchable_standup_support_fighter: None,
            unlaunchable_standup_light_fighter: None,
            unlaunchable_standup_heavy_fighter: None,
            ship_stance: None,
            overload_skill: None,
            max_type_fitted: Vec::new(),
            sec_zone_fitted: None,
            sec_zone_online: None,
            sec_zone_active: None,
        }
    }
    pub fn all_passed(&self) -> bool {
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
            && self.charge_group.is_empty()
            && self.charge_size.is_empty()
            && self.charge_volume.is_empty()
            && self.capital_module.is_none()
            && self.not_loaded_item.is_empty()
            && self.module_state.is_empty()
            && self.item_kind.is_empty()
            && self.drone_group.is_none()
            && self.fighter_count.is_empty()
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
    }
}
