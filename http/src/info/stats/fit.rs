use serde::Serialize;

use crate::{
    info::stats::details::{
        HStatCapSim, HStatDmg, HStatEhp, HStatErps, HStatHp, HStatJamApplied, HStatMining, HStatOutReps, HStatResists,
        HStatResource, HStatRps, HStatSensors, HStatSlot,
    },
    util::TriStateField,
};

#[derive(Serialize)]
pub(crate) struct HFitStats {
    // Fit output stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dps: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volley: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mps: Option<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_nps: Option<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_rps: Option<Vec<Option<HStatOutReps>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_cps: Option<Vec<Option<f64>>>,
    // Fit resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cpu: Option<HStatResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) powergrid: Option<HStatResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) calibration: Option<HStatResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drone_bay_volume: Option<HStatResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drone_bandwidth: Option<HStatResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fighter_bay_volume: Option<HStatResource>,
    // Fit slots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) high_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mid_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) low_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) turret_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launcher_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rig_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) service_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) subsystem_slots: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_drones: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_light_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_heavy_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_support_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_light_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_heavy_fighters: Option<HStatSlot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) launched_st_support_fighters: Option<HStatSlot>,
    // Ship tank
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatResists>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatHp>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<HStatEhp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatEhp>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatRps>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) erps: TriStateField<Vec<HStatErps>>,
    // Ship cap
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_amount: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_balance: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_sim: TriStateField<Vec<HStatCapSim>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) neut_resist: TriStateField<f64>,
    // Ship sensors
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) locks: TriStateField<u32>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) lock_range: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) scan_res: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sensors: TriStateField<HStatSensors>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dscan_range: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) probing_size: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) incoming_jam: TriStateField<HStatJamApplied>,
    // Ship mobility
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sig_radius: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) mass: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) warp_speed: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) max_warp_range: TriStateField<f64>,
    // Ship misc stats
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) drone_control_range: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_warp: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_jump_gate: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_jump_drive: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_dock_station: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_dock_citadel: TriStateField<bool>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_tether: TriStateField<bool>,
}
impl HFitStats {
    pub(crate) fn new() -> Self {
        Self {
            // Fit output stats
            dps: Option::default(),
            volley: Option::default(),
            mps: Option::default(),
            outgoing_nps: Option::default(),
            outgoing_rps: Option::default(),
            outgoing_cps: Option::default(),
            // Fit resources
            cpu: Option::default(),
            powergrid: Option::default(),
            calibration: Option::default(),
            drone_bay_volume: Option::default(),
            drone_bandwidth: Option::default(),
            fighter_bay_volume: Option::default(),
            // Fit slots
            high_slots: Option::default(),
            mid_slots: Option::default(),
            low_slots: Option::default(),
            turret_slots: Option::default(),
            launcher_slots: Option::default(),
            rig_slots: Option::default(),
            service_slots: Option::default(),
            subsystem_slots: Option::default(),
            launched_drones: Option::default(),
            launched_fighters: Option::default(),
            launched_light_fighters: Option::default(),
            launched_heavy_fighters: Option::default(),
            launched_support_fighters: Option::default(),
            launched_st_light_fighters: Option::default(),
            launched_st_heavy_fighters: Option::default(),
            launched_st_support_fighters: Option::default(),
            // Ship tank
            resists: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            // Ship cap
            cap_amount: TriStateField::default(),
            cap_balance: TriStateField::default(),
            cap_sim: TriStateField::default(),
            neut_resist: TriStateField::default(),
            // Ship sensors
            locks: TriStateField::default(),
            lock_range: TriStateField::default(),
            scan_res: TriStateField::default(),
            sensors: TriStateField::default(),
            dscan_range: TriStateField::default(),
            probing_size: TriStateField::default(),
            incoming_jam: TriStateField::default(),
            // Ship mobility
            speed: TriStateField::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            sig_radius: TriStateField::default(),
            mass: TriStateField::default(),
            warp_speed: TriStateField::default(),
            max_warp_range: TriStateField::default(),
            // Ship misc stats
            drone_control_range: TriStateField::default(),
            can_warp: TriStateField::default(),
            can_jump_gate: TriStateField::default(),
            can_jump_drive: TriStateField::default(),
            can_dock_station: TriStateField::default(),
            can_dock_citadel: TriStateField::default(),
            can_tether: TriStateField::default(),
        }
    }
}
