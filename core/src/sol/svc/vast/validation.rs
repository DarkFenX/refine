use crate::sol::svc::vast::{
    SolChargeGroupValFail, SolChargeSizeValFail, SolChargeVolumeValFail, SolMaxGroupValFail, SolResValFail,
    SolRigSizeValFail, SolShipLimitValFail, SolSlotIndexValFail, SolSlotValFail, SolSrqValFail,
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
}
impl SolValOptions {
    pub fn new(
        cpu: bool,
        powergrid: bool,
        calibration: bool,
        dronebay_volume: bool,
        drone_bandwidth: bool,
        rig_slots: bool,
        subsystem_slots: bool,
        launched_drones: bool,
        launched_fighters: bool,
        launched_support_fighters: bool,
        launched_light_fighters: bool,
        launched_heavy_fighters: bool,
        launched_standup_support_fighters: bool,
        launched_standup_light_fighters: bool,
        launched_standup_heavy_fighters: bool,
        turret_slots: bool,
        launcher_slots: bool,
        high_slots: bool,
        mid_slots: bool,
        low_slots: bool,
        implant_slot_index: bool,
        booster_slot_index: bool,
        subsystem_slot_index: bool,
        ship_limit: bool,
        max_group_fitted: bool,
        max_group_online: bool,
        max_group_active: bool,
        rig_size: bool,
        skill_reqs: bool,
        charge_group: bool,
        charge_size: bool,
        charge_volume: bool,
    ) -> Self {
        Self {
            cpu,
            powergrid,
            calibration,
            dronebay_volume,
            drone_bandwidth,
            rig_slots,
            subsystem_slots,
            launched_drones,
            launched_fighters,
            launched_support_fighters,
            launched_light_fighters,
            launched_heavy_fighters,
            launched_standup_support_fighters,
            launched_standup_light_fighters,
            launched_standup_heavy_fighters,
            turret_slots,
            launcher_slots,
            high_slots,
            mid_slots,
            low_slots,
            implant_slot_index,
            booster_slot_index,
            subsystem_slot_index,
            ship_limit,
            max_group_fitted,
            max_group_online,
            max_group_active,
            rig_size,
            skill_reqs,
            charge_group,
            charge_size,
            charge_volume,
        }
    }
    pub fn new_enabled() -> Self {
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
        }
    }
    pub fn new_disabled() -> Self {
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
        }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolResValFail>,
    pub powergrid: Option<SolResValFail>,
    pub calibration: Option<SolResValFail>,
    pub dronebay_volume: Option<SolResValFail>,
    pub drone_bandwidth: Option<SolResValFail>,
    pub rig_slots: Option<SolSlotValFail>,
    pub subsystem_slots: Option<SolSlotValFail>,
    pub launched_drones: Option<SolSlotValFail>,
    pub launched_fighters: Option<SolSlotValFail>,
    pub launched_support_fighters: Option<SolSlotValFail>,
    pub launched_light_fighters: Option<SolSlotValFail>,
    pub launched_heavy_fighters: Option<SolSlotValFail>,
    pub launched_standup_support_fighters: Option<SolSlotValFail>,
    pub launched_standup_light_fighters: Option<SolSlotValFail>,
    pub launched_standup_heavy_fighters: Option<SolSlotValFail>,
    pub turret_slots: Option<SolSlotValFail>,
    pub launcher_slots: Option<SolSlotValFail>,
    pub high_slots: Option<SolSlotValFail>,
    pub mid_slots: Option<SolSlotValFail>,
    pub low_slots: Option<SolSlotValFail>,
    pub implant_slot_index: Vec<SolSlotIndexValFail>,
    pub booster_slot_index: Vec<SolSlotIndexValFail>,
    pub subsystem_slot_index: Vec<SolSlotIndexValFail>,
    pub ship_limit: Option<SolShipLimitValFail>,
    pub max_group_fitted: Vec<SolMaxGroupValFail>,
    pub max_group_online: Vec<SolMaxGroupValFail>,
    pub max_group_active: Vec<SolMaxGroupValFail>,
    pub rig_size: Option<SolRigSizeValFail>,
    pub skill_reqs: Vec<SolSrqValFail>,
    pub charge_group: Vec<SolChargeGroupValFail>,
    pub charge_size: Vec<SolChargeSizeValFail>,
    pub charge_volume: Vec<SolChargeVolumeValFail>,
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
    }
}
