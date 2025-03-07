use crate::{
    defs::SolItemId,
    sol::svc::vast::{
        SolValCapitalModFail, SolValChargeGroupFail, SolValChargeSizeFail, SolValChargeVolumeFail,
        SolValDroneGroupFail, SolValFighterCountFail, SolValItemKindFail, SolValMaxGroupFail, SolValModuleStateFail,
        SolValNotLoadedItemFail, SolValResFail, SolValRigSizeFail, SolValShipLimitFail, SolValSlotCountFail,
        SolValSlotIndexFail, SolValSrqFail,
    },
};

#[derive(Clone)]
pub struct SolValOptions {
    pub cpu: SolValOption,
    pub powergrid: SolValOption,
    pub calibration: SolValOption,
    pub drone_bay_volume: SolValOption,
    pub drone_bandwidth: SolValOption,
    pub fighter_bay_volume: SolValOption,
    pub rig_slot_count: SolValOption,
    pub subsystem_slot_count: SolValOption,
    pub launched_drone_count: SolValOption,
    pub launched_fighter_count: SolValOption,
    pub launched_support_fighter_count: SolValOption,
    pub launched_light_fighter_count: SolValOption,
    pub launched_heavy_fighter_count: SolValOption,
    pub launched_standup_support_fighter_count: SolValOption,
    pub launched_standup_light_fighter_count: SolValOption,
    pub launched_standup_heavy_fighter_count: SolValOption,
    pub turret_slot_count: SolValOption,
    pub launcher_slot_count: SolValOption,
    pub high_slot_count: SolValOption,
    pub mid_slot_count: SolValOption,
    pub low_slot_count: SolValOption,
    pub implant_slot_index: SolValOption,
    pub booster_slot_index: SolValOption,
    pub subsystem_slot_index: SolValOption,
    pub ship_limit: SolValOption,
    pub max_group_fitted: SolValOption,
    pub max_group_online: SolValOption,
    pub max_group_active: SolValOption,
    pub rig_size: SolValOption,
    pub skill_reqs: SolValOption,
    pub charge_group: SolValOption,
    pub charge_size: SolValOption,
    pub charge_volume: SolValOption,
    pub capital_module: SolValOption,
    pub not_loaded_item: SolValOption,
    pub module_state: SolValOption,
    pub item_kind: SolValOption,
    pub drone_group: SolValOption,
    pub fighter_count: SolValOption,
}
impl SolValOptions {
    pub fn all_enabled() -> Self {
        Self {
            cpu: SolValOption::enabled(),
            powergrid: SolValOption::enabled(),
            calibration: SolValOption::enabled(),
            drone_bay_volume: SolValOption::enabled(),
            drone_bandwidth: SolValOption::enabled(),
            fighter_bay_volume: SolValOption::enabled(),
            rig_slot_count: SolValOption::enabled(),
            subsystem_slot_count: SolValOption::enabled(),
            launched_drone_count: SolValOption::enabled(),
            launched_fighter_count: SolValOption::enabled(),
            launched_support_fighter_count: SolValOption::enabled(),
            launched_light_fighter_count: SolValOption::enabled(),
            launched_heavy_fighter_count: SolValOption::enabled(),
            launched_standup_support_fighter_count: SolValOption::enabled(),
            launched_standup_light_fighter_count: SolValOption::enabled(),
            launched_standup_heavy_fighter_count: SolValOption::enabled(),
            turret_slot_count: SolValOption::enabled(),
            launcher_slot_count: SolValOption::enabled(),
            high_slot_count: SolValOption::enabled(),
            mid_slot_count: SolValOption::enabled(),
            low_slot_count: SolValOption::enabled(),
            implant_slot_index: SolValOption::enabled(),
            booster_slot_index: SolValOption::enabled(),
            subsystem_slot_index: SolValOption::enabled(),
            ship_limit: SolValOption::enabled(),
            max_group_fitted: SolValOption::enabled(),
            max_group_online: SolValOption::enabled(),
            max_group_active: SolValOption::enabled(),
            rig_size: SolValOption::enabled(),
            skill_reqs: SolValOption::enabled(),
            charge_group: SolValOption::enabled(),
            charge_size: SolValOption::enabled(),
            charge_volume: SolValOption::enabled(),
            capital_module: SolValOption::enabled(),
            not_loaded_item: SolValOption::enabled(),
            module_state: SolValOption::enabled(),
            item_kind: SolValOption::enabled(),
            drone_group: SolValOption::enabled(),
            fighter_count: SolValOption::enabled(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            cpu: SolValOption::disabled(),
            powergrid: SolValOption::disabled(),
            calibration: SolValOption::disabled(),
            drone_bay_volume: SolValOption::disabled(),
            drone_bandwidth: SolValOption::disabled(),
            fighter_bay_volume: SolValOption::disabled(),
            rig_slot_count: SolValOption::disabled(),
            subsystem_slot_count: SolValOption::disabled(),
            launched_drone_count: SolValOption::disabled(),
            launched_fighter_count: SolValOption::disabled(),
            launched_support_fighter_count: SolValOption::disabled(),
            launched_light_fighter_count: SolValOption::disabled(),
            launched_heavy_fighter_count: SolValOption::disabled(),
            launched_standup_support_fighter_count: SolValOption::disabled(),
            launched_standup_light_fighter_count: SolValOption::disabled(),
            launched_standup_heavy_fighter_count: SolValOption::disabled(),
            turret_slot_count: SolValOption::disabled(),
            launcher_slot_count: SolValOption::disabled(),
            high_slot_count: SolValOption::disabled(),
            mid_slot_count: SolValOption::disabled(),
            low_slot_count: SolValOption::disabled(),
            implant_slot_index: SolValOption::disabled(),
            booster_slot_index: SolValOption::disabled(),
            subsystem_slot_index: SolValOption::disabled(),
            ship_limit: SolValOption::disabled(),
            max_group_fitted: SolValOption::disabled(),
            max_group_online: SolValOption::disabled(),
            max_group_active: SolValOption::disabled(),
            rig_size: SolValOption::disabled(),
            skill_reqs: SolValOption::disabled(),
            charge_group: SolValOption::disabled(),
            charge_size: SolValOption::disabled(),
            charge_volume: SolValOption::disabled(),
            capital_module: SolValOption::disabled(),
            not_loaded_item: SolValOption::disabled(),
            module_state: SolValOption::disabled(),
            item_kind: SolValOption::disabled(),
            drone_group: SolValOption::disabled(),
            fighter_count: SolValOption::disabled(),
        }
    }
}

#[derive(Clone)]
pub struct SolValOption {
    pub enabled: bool,
    pub ignored_item_ids: Vec<SolItemId>,
}
impl SolValOption {
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ignored_item_ids: Vec::new(),
        }
    }
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ignored_item_ids: Vec::new(),
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
