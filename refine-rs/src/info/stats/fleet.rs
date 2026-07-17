use crate::{
    PValue,
    stats::{StatDmg, StatMining, StatOutReps},
};

pub struct FleetStats {
    pub dmg: Option<Vec<Option<StatDmg>>> = None,
    pub mps: Option<Vec<StatMining>> = None,
    pub outgoing_nps: Option<Vec<Option<PValue>>> = None,
    pub outgoing_rps: Option<Vec<Option<StatOutReps>>> = None,
    pub outgoing_cps: Option<Vec<Option<PValue>>> = None,
    pub mass: Option<Vec<PValue>> = None,
}
