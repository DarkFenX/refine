pub use crate::{
    PValue, TriStateField, UnitInterval,
    stats::{StatDmg, StatEhp, StatErps, StatHp, StatMining, StatOutReps, StatResists, StatRps},
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
}
