use crate::{
    Count, PValue, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatResource, StatResult, StatRps, StatSensors, StatSlot,
        err::{AgilityStatError, FitShipStatError},
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitStats {
    // Fit output stats
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub dmg: Vec<Option<StatDmg>> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub mps: Vec<StatMining> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub outgoing_nps: Vec<Option<PValue>> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub outgoing_rps: Vec<Option<StatOutReps>> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub outgoing_cps: Vec<Option<PValue>> = Vec::new(),
    // Fit resources
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cpu: StatResult<StatResource, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub powergrid: StatResult<StatResource, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub calibration: StatResult<StatResource, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub drone_bay_volume: StatResult<StatResource, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub drone_bandwidth: StatResult<StatResource, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub fighter_bay_volume: StatResult<StatResource, !, !> = StatResult::NotRequested,
    // Fit slots
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub high_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mid_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub low_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub turret_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launcher_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub rig_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub service_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub subsystem_slots: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_drones: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_light_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_heavy_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_support_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_st_light_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_st_heavy_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub launched_st_support_fighters: StatResult<StatSlot, !, !> = StatResult::NotRequested,
    // Ship tank
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
    // Ship cap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_amount: Option<Vec<PValue>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_balance: Option<Vec<Option<Value>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub cap_sim: Option<Vec<Option<StatCapSim>>> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_stat"))]
    pub neut_resist: Option<Vec<UnitInterval>> = None,
    // Ship sensors
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
    // Ship mobility
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub speed: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub agility: StatResult<PValue, FitShipStatError<AgilityStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub align_time: StatResult<PValue, FitShipStatError<AgilityStatError>, !> = StatResult::NotRequested,
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
    // Ship misc stats
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
