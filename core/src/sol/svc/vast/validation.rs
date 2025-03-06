use crate::sol::svc::vast::{
    SolValCapitalModFail, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail, SolValDroneGroupFail,
    SolValItemKindFail, SolValMaxGroupFail, SolValModuleStateFail, SolValNotLoadedItemFail, SolValResFail,
    SolValRigSizeFail, SolValShipLimitFail, SolValSlotFail, SolValSlotIndexFail, SolValSrqFail,
};

#[derive(Copy, Clone)]
pub struct SolValOptions {
    pub cpu: bool,
    pub powergrid: bool,
    pub calibration: bool,
    pub dronebay_volume: bool,
    pub drone_bandwidth: bool,
    pub rig_slots: bool,
    pub subsystem_slots: bool,
    pub launched_drones: bool,
    pub launched_fighters: bool,
    pub launched_support_fighters: bool,
    pub launched_light_fighters: bool,
    pub launched_heavy_fighters: bool,
    pub launched_standup_support_fighters: bool,
    pub launched_standup_light_fighters: bool,
    pub launched_standup_heavy_fighters: bool,
    pub turret_slots: bool,
    pub launcher_slots: bool,
    pub high_slots: bool,
    pub mid_slots: bool,
    pub low_slots: bool,
    pub implant_slot_index: bool,
    pub booster_slot_index: bool,
    pub subsystem_slot_index: bool,
    pub ship_limit: bool,
    pub max_group_fitted: bool,
    pub max_group_online: bool,
    pub max_group_active: bool,
    pub rig_size: bool,
    pub skill_reqs: bool,
    pub charge_group: bool,
    pub charge_size: bool,
    pub charge_volume: bool,
    pub capital_module: bool,
    pub not_loaded_item: bool,
    pub module_state: bool,
    pub item_kind: bool,
    pub drone_group: bool,
}
impl SolValOptions {
    pub fn new_all_enabled() -> Self {
        Self {
            cpu: true,
            powergrid: true,
            calibration: true,
            dronebay_volume: true,
            drone_bandwidth: true,
            rig_slots: true,
            subsystem_slots: true,
            launched_drones: true,
            launched_fighters: true,
            launched_support_fighters: true,
            launched_light_fighters: true,
            launched_heavy_fighters: true,
            launched_standup_support_fighters: true,
            launched_standup_light_fighters: true,
            launched_standup_heavy_fighters: true,
            turret_slots: true,
            launcher_slots: true,
            high_slots: true,
            mid_slots: true,
            low_slots: true,
            implant_slot_index: true,
            booster_slot_index: true,
            subsystem_slot_index: true,
            ship_limit: true,
            max_group_fitted: true,
            max_group_online: true,
            max_group_active: true,
            rig_size: true,
            skill_reqs: true,
            charge_group: true,
            charge_size: true,
            charge_volume: true,
            capital_module: true,
            not_loaded_item: true,
            module_state: true,
            item_kind: true,
            drone_group: true,
        }
    }
    pub fn new_all_disabled() -> Self {
        Self {
            cpu: false,
            powergrid: false,
            calibration: false,
            dronebay_volume: false,
            drone_bandwidth: false,
            rig_slots: false,
            subsystem_slots: false,
            launched_drones: false,
            launched_fighters: false,
            launched_support_fighters: false,
            launched_light_fighters: false,
            launched_heavy_fighters: false,
            launched_standup_support_fighters: false,
            launched_standup_light_fighters: false,
            launched_standup_heavy_fighters: false,
            turret_slots: false,
            launcher_slots: false,
            high_slots: false,
            mid_slots: false,
            low_slots: false,
            implant_slot_index: false,
            booster_slot_index: false,
            subsystem_slot_index: false,
            ship_limit: false,
            max_group_fitted: false,
            max_group_online: false,
            max_group_active: false,
            rig_size: false,
            skill_reqs: false,
            charge_group: false,
            charge_size: false,
            charge_volume: false,
            capital_module: false,
            not_loaded_item: false,
            module_state: false,
            item_kind: false,
            drone_group: false,
        }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolValResFail>,
    pub powergrid: Option<SolValResFail>,
    pub calibration: Option<SolValResFail>,
    pub dronebay_volume: Option<SolValResFail>,
    pub drone_bandwidth: Option<SolValResFail>,
    pub rig_slots: Option<SolValSlotFail>,
    pub subsystem_slots: Option<SolValSlotFail>,
    pub launched_drones: Option<SolValSlotFail>,
    pub launched_fighters: Option<SolValSlotFail>,
    pub launched_support_fighters: Option<SolValSlotFail>,
    pub launched_light_fighters: Option<SolValSlotFail>,
    pub launched_heavy_fighters: Option<SolValSlotFail>,
    pub launched_standup_support_fighters: Option<SolValSlotFail>,
    pub launched_standup_light_fighters: Option<SolValSlotFail>,
    pub launched_standup_heavy_fighters: Option<SolValSlotFail>,
    pub turret_slots: Option<SolValSlotFail>,
    pub launcher_slots: Option<SolValSlotFail>,
    pub high_slots: Option<SolValSlotFail>,
    pub mid_slots: Option<SolValSlotFail>,
    pub low_slots: Option<SolValSlotFail>,
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
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            cpu: None,
            powergrid: None,
            calibration: None,
            dronebay_volume: None,
            drone_bandwidth: None,
            rig_slots: None,
            subsystem_slots: None,
            launched_drones: None,
            launched_fighters: None,
            launched_support_fighters: None,
            launched_light_fighters: None,
            launched_heavy_fighters: None,
            launched_standup_support_fighters: None,
            launched_standup_light_fighters: None,
            launched_standup_heavy_fighters: None,
            turret_slots: None,
            launcher_slots: None,
            high_slots: None,
            mid_slots: None,
            low_slots: None,
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
        }
    }
    pub fn all_passed(&self) -> bool {
        self.cpu.is_none()
            && self.powergrid.is_none()
            && self.calibration.is_none()
            && self.dronebay_volume.is_none()
            && self.drone_bandwidth.is_none()
            && self.rig_slots.is_none()
            && self.subsystem_slots.is_none()
            && self.launched_drones.is_none()
            && self.launched_fighters.is_none()
            && self.launched_support_fighters.is_none()
            && self.launched_light_fighters.is_none()
            && self.launched_heavy_fighters.is_none()
            && self.launched_standup_support_fighters.is_none()
            && self.launched_standup_light_fighters.is_none()
            && self.launched_standup_heavy_fighters.is_none()
            && self.turret_slots.is_none()
            && self.launcher_slots.is_none()
            && self.high_slots.is_none()
            && self.mid_slots.is_none()
            && self.low_slots.is_none()
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
    }
}
