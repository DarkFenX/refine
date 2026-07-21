pub use crate::{
    Count, PValue, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatResult, StatRps, StatSensors,
        err::{
            AgilityStatError, ItemAppliedStatError, ItemStatError, JumpStatError, MaxWarpRangeStatError,
            ProbingSizeStatError, WarpSpeedStatError,
        },
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStats {
    // Output
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dmg: StatResult<StatDmg, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mps: StatResult<StatMining, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_nps: StatResult<PValue, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_rps: StatResult<StatOutReps, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_cps: StatResult<PValue, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    // Tank
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub resists: StatResult<StatResists, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub hp: StatResult<StatHp, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub ehp: StatResult<StatEhp, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub wc_ehp: StatResult<StatEhp, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub rps: StatResult<StatRps, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub erps: StatResult<StatErps, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub breach_resist: StatResult<UnitInterval, ItemStatError<!>, !> = StatResult::NotRequested,
    // Cap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_amount: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_balance: StatResult<Value, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_sim: StatResult<StatCapSim, ItemAppliedStatError<!>, ItemAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub neut_resist: StatResult<UnitInterval, ItemStatError<!>, !> = StatResult::NotRequested,
    // Sensors
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub locks: StatResult<Count, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub lock_range: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub scan_res: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sensors: StatResult<StatSensors, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dscan_range: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub probing_size: StatResult<PValue, ItemStatError<ProbingSizeStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub incoming_jam: StatResult<StatInJam, ItemStatError<!>, !> = StatResult::NotRequested,
    // Mobility
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub speed: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub agility: StatResult<PValue, ItemStatError<AgilityStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub align_time: StatResult<PValue, ItemStatError<AgilityStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sig_radius: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mass: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub warp_speed: StatResult<PValue, ItemStatError<WarpSpeedStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub max_warp_range: StatResult<PValue, ItemStatError<MaxWarpRangeStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub jump: StatResult<StatJump, ItemStatError<JumpStatError>, !> = StatResult::NotRequested,
    // Misc
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub drone_control_range: StatResult<PValue, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_warp: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_gate: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_wormhole: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_drive: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_station: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_citadel: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_tether: StatResult<bool, ItemStatError<!>, !> = StatResult::NotRequested,
}
