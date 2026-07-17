pub use crate::{
    PValue, TriStateField, UnitInterval, Value,
    stats::{StatCapSim, StatDmg, StatEhp, StatErps, StatHp, StatJump, StatMining, StatOutReps, StatResists, StatRps},
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
    // Mobility
    pub speed: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub agility: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub align_time: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub sig_radius: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub mass: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub warp_speed: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub max_warp_range: TriStateField<Vec<PValue>> = TriStateField::Absent,
    pub jump: TriStateField<Vec<StatJump>> = TriStateField::Absent,
}
