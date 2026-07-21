pub use crate::{
    Count, PValue, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatResult, StatRps, StatSensors, err::ItemStatError,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStats {
    // Output
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub dmg: Option<Vec<Option<StatDmg>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub mps: Option<Vec<StatMining>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub outgoing_nps: Option<Vec<Option<PValue>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub outgoing_rps: Option<Vec<Option<StatOutReps>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub outgoing_cps: Option<Vec<Option<PValue>>> = None,
    // Tank
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub resists: Option<Vec<StatResists>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub hp: Option<Vec<StatHp>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub ehp: Option<Vec<StatEhp>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub wc_ehp: Option<Vec<StatEhp>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub rps: Option<Vec<StatRps>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub erps: Option<Vec<StatErps>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub breach_resist: Option<Vec<UnitInterval>> = None,
    // Cap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_amount: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_balance: Option<Vec<Option<Value>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_sim: Option<Vec<Option<StatCapSim>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub neut_resist: Option<Vec<UnitInterval>> = None,
    // Sensors
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub locks: Option<Vec<Count>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub lock_range: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub scan_res: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub sensors: Option<Vec<StatSensors>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub dscan_range: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub probing_size: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub incoming_jam: Option<Vec<StatInJam>> = None,
    // Mobility
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub speed: StatResult<PValue, ItemStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub agility: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub align_time: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub sig_radius: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub mass: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub warp_speed: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub max_warp_range: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub jump: Option<Vec<StatJump>> = None,
    // Misc
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub drone_control_range: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_warp: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_jump_gate: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_jump_wormhole: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_jump_drive: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_dock_station: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_dock_citadel: Option<Vec<bool>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub can_tether: Option<Vec<bool>> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn skip_stat<T>(details: &Option<Vec<T>>) -> bool {
    match details {
        Some(details) => details.is_empty(),
        None => true,
    }
}
