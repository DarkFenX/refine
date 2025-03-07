use crate::sol::svc::vast::{
    SolValCapitalModFail, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail, SolValDroneGroupFail,
    SolValFighterCountFail, SolValItemKindFail, SolValMaxGroupFail, SolValModuleStateFail, SolValNotLoadedItemFail,
    SolValResFail, SolValRigSizeFail, SolValShipLimitFail, SolValSlotCountFail, SolValSlotIndexFail, SolValSrqFail,
};

#[derive(Copy, Clone)]
pub struct SolValOptions {
    pub cpu: bool,
    pub powergrid: bool,
    pub calibration: bool,
    pub drone_bay_volume: bool,
    pub drone_bandwidth: bool,
    pub fighter_bay_volume: bool,
    pub rig_slot_count: bool,
    pub subsystem_slot_count: bool,
    pub launched_drone_count: bool,
    pub launched_fighter_count: bool,
    pub launched_support_fighter_count: bool,
    pub launched_light_fighter_count: bool,
    pub launched_heavy_fighter_count: bool,
    pub launched_standup_support_fighter_count: bool,
    pub launched_standup_light_fighter_count: bool,
    pub launched_standup_heavy_fighter_count: bool,
    pub turret_slot_count: bool,
    pub launcher_slot_count: bool,
    pub high_slot_count: bool,
    pub mid_slot_count: bool,
    pub low_slot_count: bool,
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
    pub fighter_count: bool,
}
impl SolValOptions {
    pub fn new_all_enabled() -> Self {
        Self {
            cpu: true,
            powergrid: true,
            calibration: true,
            drone_bay_volume: true,
            drone_bandwidth: true,
            fighter_bay_volume: true,
            rig_slot_count: true,
            subsystem_slot_count: true,
            launched_drone_count: true,
            launched_fighter_count: true,
            launched_support_fighter_count: true,
            launched_light_fighter_count: true,
            launched_heavy_fighter_count: true,
            launched_standup_support_fighter_count: true,
            launched_standup_light_fighter_count: true,
            launched_standup_heavy_fighter_count: true,
            turret_slot_count: true,
            launcher_slot_count: true,
            high_slot_count: true,
            mid_slot_count: true,
            low_slot_count: true,
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
            fighter_count: true,
        }
    }
    pub fn new_all_disabled() -> Self {
        Self {
            cpu: false,
            powergrid: false,
            calibration: false,
            drone_bay_volume: false,
            drone_bandwidth: false,
            fighter_bay_volume: false,
            rig_slot_count: false,
            subsystem_slot_count: false,
            launched_drone_count: false,
            launched_fighter_count: false,
            launched_support_fighter_count: false,
            launched_light_fighter_count: false,
            launched_heavy_fighter_count: false,
            launched_standup_support_fighter_count: false,
            launched_standup_light_fighter_count: false,
            launched_standup_heavy_fighter_count: false,
            turret_slot_count: false,
            launcher_slot_count: false,
            high_slot_count: false,
            mid_slot_count: false,
            low_slot_count: false,
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
            fighter_count: false,
        }
    }
}

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
