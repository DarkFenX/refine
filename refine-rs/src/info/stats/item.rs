pub use crate::{
    PValue, TriStateField,
    stats::{StatDmg, StatMining, StatOutReps},
};

pub struct ItemStats {
    // Output
    pub dmg: TriStateField<Vec<Option<StatDmg>>> = TriStateField::Absent,
    pub mps: TriStateField<Vec<StatMining>> = TriStateField::Absent,
    pub outgoing_nps: TriStateField<Vec<Option<PValue>>> = TriStateField::Absent,
    pub outgoing_rps: TriStateField<Vec<Option<StatOutReps>>> = TriStateField::Absent,
    pub outgoing_cps: TriStateField<Vec<Option<PValue>>> = TriStateField::Absent,
}
