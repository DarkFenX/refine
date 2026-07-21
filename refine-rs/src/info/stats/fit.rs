use crate::{
    Count, PValue, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatResource, StatResult, StatRps, StatSensors, StatSlot,
        err::{
            AgilityStatError, FitAppliedStatError, FitCharacterStatError, FitShipAppliedStatError, FitShipStatError,
            JumpStatError, MaxWarpRangeStatError, ProbingSizeStatError, WarpSpeedStatError,
        },
    },
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitStats {
    // Fit output stats
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dmg: StatResult<StatDmg, !, FitAppliedStatError> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mps: StatResult<StatMining, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_nps: StatResult<PValue, !, FitAppliedStatError> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_rps: StatResult<StatOutReps, !, FitAppliedStatError> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_cps: StatResult<PValue, !, FitAppliedStatError> = StatResult::NotRequested,
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub resists: StatResult<StatResists, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub hp: StatResult<StatHp, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub ehp: StatResult<StatEhp, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub wc_ehp: StatResult<StatEhp, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub rps: StatResult<StatRps, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub erps: StatResult<StatErps, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub breach_resist: StatResult<UnitInterval, FitShipStatError<!>, !> = StatResult::NotRequested,
    // Ship cap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_amount: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_balance: StatResult<Value, FitShipAppliedStatError<!>, FitShipAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub cap_sim: StatResult<StatCapSim, FitShipAppliedStatError<!>, FitShipAppliedStatError<!>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub neut_resist: StatResult<UnitInterval, FitShipStatError<!>, !> = StatResult::NotRequested,
    // Ship sensors
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub locks: StatResult<Count, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub lock_range: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub scan_res: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sensors: StatResult<StatSensors, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dscan_range: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub probing_size: StatResult<PValue, FitShipStatError<ProbingSizeStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub incoming_jam: StatResult<StatInJam, FitShipStatError<!>, !> = StatResult::NotRequested,
    // Ship mobility
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub speed: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub agility: StatResult<PValue, FitShipStatError<AgilityStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub align_time: StatResult<PValue, FitShipStatError<AgilityStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub sig_radius: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mass: StatResult<PValue, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub warp_speed: StatResult<PValue, FitShipStatError<WarpSpeedStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub max_warp_range: StatResult<PValue, FitShipStatError<MaxWarpRangeStatError>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub jump: StatResult<StatJump, FitShipStatError<JumpStatError>, !> = StatResult::NotRequested,
    // Ship misc stats
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub drone_control_range: StatResult<PValue, FitCharacterStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_warp: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_gate: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_wormhole: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_jump_drive: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_station: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_dock_citadel: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub can_tether: StatResult<bool, FitShipStatError<!>, !> = StatResult::NotRequested,
}
