pub use crate::{
    Count, PValue, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatResult, StatRps, StatSensors,
        err::{
            StatAgilityError, StatItemAppliedError, StatItemError, StatJumpError, StatMaxWarpRangeError,
            StatProbingSizeError, StatWarpSpeedError,
        },
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ItemStatsResult {
    // Output
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dmg: StatResult<StatDmg, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mps: StatResult<StatMining, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_nps: StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_rps: StatResult<StatOutReps, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_cps: StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    // Tank
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub resists: StatResult<StatResists, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub hp: StatResult<StatHp, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub ehp: StatResult<StatEhp, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub wc_ehp: StatResult<StatEhp, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub rps: StatResult<StatRps, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub erps: StatResult<StatErps, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub breach_resist: StatResult<UnitInterval, StatItemError<!>, !> = StatResult::NotRequested,
    // Cap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_amount: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_balance: StatResult<Value, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_sim: StatResult<StatCapSim, StatItemAppliedError<!>, StatItemAppliedError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub neut_resist: StatResult<UnitInterval, StatItemError<!>, !> = StatResult::NotRequested,
    // Sensors
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub locks: StatResult<Count, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub lock_range: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub scan_res: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sensors: StatResult<StatSensors, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dscan_range: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub probing_size: StatResult<PValue, StatItemError<StatProbingSizeError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub incoming_jam: StatResult<StatInJam, StatItemError<!>, !> = StatResult::NotRequested,
    // Mobility
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub speed: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub agility: StatResult<PValue, StatItemError<StatAgilityError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub align_time: StatResult<PValue, StatItemError<StatAgilityError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sig_radius: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mass: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub warp_speed: StatResult<PValue, StatItemError<StatWarpSpeedError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub max_warp_range: StatResult<PValue, StatItemError<StatMaxWarpRangeError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub jump: StatResult<StatJump, StatItemError<StatJumpError>, !> = StatResult::NotRequested,
    // Misc
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub drone_control_range: StatResult<PValue, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_warp: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_gate: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_wormhole: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_drive: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_station: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_citadel: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_tether: StatResult<bool, StatItemError<!>, !> = StatResult::NotRequested,
}
