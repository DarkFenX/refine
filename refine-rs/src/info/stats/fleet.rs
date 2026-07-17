use crate::{
    PValue, Value,
    stats::{StatMining, StatOutReps},
};

pub struct FleetStats {
    pub dmg: Option<Vec<Option<Value>>> = None,
    pub mps: Option<Vec<StatMining>> = None,
    pub outgoing_nps: Option<Vec<Option<PValue>>> = None,
    pub outgoing_rps: Option<Vec<Option<StatOutReps>>> = None,
    pub outgoing_cps: Option<Vec<Option<PValue>>> = None,
    pub mass: Option<Vec<PValue>> = None,
}
