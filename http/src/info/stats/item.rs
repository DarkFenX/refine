use serde::Serialize;

use crate::{
    info::stats::details::{
        HStatCapSim, HStatDmg, HStatEhp, HStatErps, HStatHp, HStatInJam, HStatMining, HStatOutReps, HStatResists,
        HStatRps, HStatSensors,
    },
    util::TriStateField,
};

#[derive(Serialize)]
pub(crate) struct HItemStats {
    // Output
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dps: TriStateField<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) volley: TriStateField<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) mps: TriStateField<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_nps: TriStateField<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_rps: TriStateField<Vec<Option<HStatOutReps>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) outgoing_cps: TriStateField<Vec<Option<f64>>>,
    // Tank
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
    // Cap
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_amount: TriStateField<f64>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_balance: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_sim: TriStateField<Vec<HStatCapSim>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) neut_resist: TriStateField<f64>,
    // Sensors
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
    pub(crate) incoming_jam: TriStateField<HStatInJam>,
    // Mobility
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
    // Misc
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
impl HItemStats {
    pub(crate) fn new() -> Self {
        Self {
            // Output
            dps: TriStateField::default(),
            volley: TriStateField::default(),
            mps: TriStateField::default(),
            outgoing_nps: TriStateField::default(),
            outgoing_rps: TriStateField::default(),
            outgoing_cps: TriStateField::default(),
            // Tank
            resists: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            // Cap
            cap_amount: TriStateField::default(),
            cap_balance: TriStateField::default(),
            cap_sim: TriStateField::default(),
            neut_resist: TriStateField::default(),
            // Sensors
            locks: TriStateField::default(),
            lock_range: TriStateField::default(),
            scan_res: TriStateField::default(),
            sensors: TriStateField::default(),
            dscan_range: TriStateField::default(),
            probing_size: TriStateField::default(),
            incoming_jam: TriStateField::default(),
            // Mobility
            speed: TriStateField::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            sig_radius: TriStateField::default(),
            mass: TriStateField::default(),
            warp_speed: TriStateField::default(),
            max_warp_range: TriStateField::default(),
            // Misc
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
