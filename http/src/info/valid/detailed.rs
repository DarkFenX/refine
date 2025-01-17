use crate::info::valid::details::{HResValFail, HSlotValFail};

#[derive(serde::Serialize)]
pub(crate) struct HValidInfoDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HValidInfoDetails::is_empty")]
    details: HValidInfoDetails,
}
impl From<&rc::SolValResult> for HValidInfoDetailed {
    fn from(core_val_result: &rc::SolValResult) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: core_val_result.into(),
        }
    }
}

#[derive(serde::Serialize)]
struct HValidInfoDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    powergrid: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dronebay_volume: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drone_bandwidth: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rig_slots: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subsystem_slots: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_drones: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_support_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_light_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_heavy_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_support_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_light_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launched_standup_heavy_fighters: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turret_slots: Option<HSlotValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launcher_slots: Option<HSlotValFail>,
}
impl HValidInfoDetails {
    fn is_empty(&self) -> bool {
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
impl From<&rc::SolValResult> for HValidInfoDetails {
    fn from(core_val_result: &rc::SolValResult) -> Self {
        Self {
            cpu: core_val_result.cpu.as_ref().map(|v| v.into()),
            powergrid: core_val_result.powergrid.as_ref().map(|v| v.into()),
            calibration: core_val_result.calibration.as_ref().map(|v| v.into()),
            dronebay_volume: core_val_result.dronebay_volume.as_ref().map(|v| v.into()),
            drone_bandwidth: core_val_result.drone_bandwidth.as_ref().map(|v| v.into()),
            rig_slots: core_val_result.rig_slots.as_ref().map(|v| v.into()),
            subsystem_slots: core_val_result.subsystem_slots.as_ref().map(|v| v.into()),
            launched_drones: core_val_result.launched_drones.as_ref().map(|v| v.into()),
            launched_fighters: core_val_result.launched_fighters.as_ref().map(|v| v.into()),
            launched_support_fighters: core_val_result.launched_support_fighters.as_ref().map(|v| v.into()),
            launched_light_fighters: core_val_result.launched_light_fighters.as_ref().map(|v| v.into()),
            launched_heavy_fighters: core_val_result.launched_heavy_fighters.as_ref().map(|v| v.into()),
            launched_standup_support_fighters: core_val_result
                .launched_standup_support_fighters
                .as_ref()
                .map(|v| v.into()),
            launched_standup_light_fighters: core_val_result
                .launched_standup_light_fighters
                .as_ref()
                .map(|v| v.into()),
            launched_standup_heavy_fighters: core_val_result
                .launched_standup_heavy_fighters
                .as_ref()
                .map(|v| v.into()),
            turret_slots: core_val_result.turret_slots.as_ref().map(|v| v.into()),
            launcher_slots: core_val_result.launcher_slots.as_ref().map(|v| v.into()),
        }
    }
}
