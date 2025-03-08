use crate::sol::svc::vast::{
    SolValCapitalModFail, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail, SolValDroneGroupFail,
    SolValFighterCountFail, SolValItemKindFail, SolValMaxGroupFail, SolValModuleStateFail, SolValNotLoadedItemFail,
    SolValResFail, SolValRigSizeFail, SolValShipLimitFail, SolValSlotCountFail, SolValSlotIndexFail, SolValSrqFail,
};

pub struct SolValResult {
    pub cpu: Option<SolValResFail>,
    pub powergrid: Option<SolValResFail>,
    pub calibration: Option<SolValResFail>,
    pub drone_bay_volume: Option<SolValResFail>,
    pub drone_bandwidth: Option<SolValResFail>,
    pub fighter_bay_volume: Option<SolValResFail>,
    pub rig_slot_count: Option<SolValSlotCountFail>,
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
    }
}
