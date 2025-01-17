use crate::sol::svc::vast::{SolResValFail, SolSlotValFail};

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
    }
}
