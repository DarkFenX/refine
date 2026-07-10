use serde::Serialize;

use crate::{
    info::stats::details::{
        HStatCapSim, HStatDmg, HStatEhp, HStatErps, HStatHp, HStatInJam, HStatJump, HStatMining, HStatOutReps,
        HStatResists, HStatRps, HStatSensors,
    },
    util::TriStateField,
};

#[derive(Serialize)]
pub(crate) struct HItemStats {
    // Output
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dmg: TriStateField<Vec<Option<HStatDmg>>>,
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
    pub(crate) resists: TriStateField<Vec<HStatResists>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<Vec<HStatHp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<HStatEhp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<Vec<HStatEhp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatRps>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) erps: TriStateField<Vec<HStatErps>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) breach_resist: TriStateField<Vec<f64>>,
    // Cap
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_amount: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_balance: TriStateField<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) cap_sim: TriStateField<Vec<Option<HStatCapSim>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) neut_resist: TriStateField<Vec<f64>>,
    // Sensors
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) locks: TriStateField<Vec<u32>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) lock_range: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) scan_res: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sensors: TriStateField<Vec<HStatSensors>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dscan_range: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) probing_size: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) incoming_jam: TriStateField<Vec<HStatInJam>>,
    // Mobility
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sig_radius: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) mass: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) warp_speed: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) max_warp_range: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) jump: TriStateField<Vec<HStatJump>>,
    // Misc
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) drone_control_range: TriStateField<Vec<f64>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_warp: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_jump_gate: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_jump_wormhole: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_jump_drive: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_dock_station: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_dock_citadel: TriStateField<Vec<bool>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) can_tether: TriStateField<Vec<bool>>,
}
impl HItemStats {
    pub(crate) fn new() -> Self {
        Self {
            // Output
            dmg: TriStateField::default(),
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
            breach_resist: TriStateField::default(),
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
            jump: TriStateField::default(),
            // Misc
            drone_control_range: TriStateField::default(),
            can_warp: TriStateField::default(),
            can_jump_gate: TriStateField::default(),
            can_jump_wormhole: TriStateField::default(),
            can_jump_drive: TriStateField::default(),
            can_dock_station: TriStateField::default(),
            can_dock_citadel: TriStateField::default(),
            can_tether: TriStateField::default(),
        }
    }
}
