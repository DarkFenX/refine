pub use crate::{
    Count, PValue, TriStateField, UnitInterval, Value,
    stats::{
        StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatInJam, StatJump, StatMining, StatOutReps, StatResists,
        StatRps, StatSensors,
    },
};

pub struct ItemStats {
    // Output
    pub dmg: TriStateField<Vec<Option<StatDmg>>> = TriStateField::Absent,
    pub mps: TriStateField<Vec<StatMining>> = TriStateField::Absent,
    pub outgoing_nps: TriStateField<Vec<Option<PValue>>> = TriStateField::Absent,
    pub outgoing_rps: TriStateField<Vec<Option<StatOutReps>>> = TriStateField::Absent,
    pub outgoing_cps: TriStateField<Vec<Option<PValue>>> = TriStateField::Absent,
    // Tank
    pub resists: TriStateField<Vec<StatResists>> = TriStateField::Absent,
    pub hp: TriStateField<Vec<StatHp>> = TriStateField::Absent,
    pub ehp: TriStateField<Vec<StatEhp>> = TriStateField::Absent,
    pub wc_ehp: TriStateField<Vec<StatEhp>> = TriStateField::Absent,
    pub rps: TriStateField<Vec<StatRps>> = TriStateField::Absent,
    pub erps: TriStateField<Vec<StatErps>> = TriStateField::Absent,
    pub breach_resist: TriStateField<Vec<UnitInterval>> = TriStateField::Absent,
    // Cap
    pub cap_amount: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub cap_balance: TriStateField<Vec<Option<Value>>> = TriStateField::Absent,
    pub cap_sim: TriStateField<Vec<Option<StatCapSim>>> = TriStateField::Absent,
    pub neut_resist: TriStateField<Vec<UnitInterval>> = TriStateField::Absent,
    // Sensors
    pub locks: TriStateField<Vec<Count>> = TriStateField::Absent,
    pub lock_range: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub scan_res: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub sensors: TriStateField<Vec<StatSensors>> = TriStateField::Absent,
    pub dscan_range: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub probing_size: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub incoming_jam: TriStateField<Vec<StatInJam>> = TriStateField::Absent,
    // Mobility
    pub speed: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub agility: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub align_time: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub sig_radius: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub mass: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub warp_speed: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub max_warp_range: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub jump: TriStateField<Vec<StatJump>> = TriStateField::Absent,
    // Misc
    pub drone_control_range: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub can_warp: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_jump_gate: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_jump_wormhole: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_jump_drive: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_dock_station: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_dock_citadel: TriStateField<Vec<bool>> = TriStateField::Absent,
    pub can_tether: TriStateField<Vec<bool>> = TriStateField::Absent,
}
