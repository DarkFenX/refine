use crate::{
    PValue,
    stats::{StatDmg, StatMining, StatOutReps, StatResource},
};

pub struct FitStats {
    // Fit output stats
    pub dmg: Option<Vec<Option<StatDmg>>> = None,
    pub mps: Option<Vec<StatMining>> = None,
    pub outgoing_nps: Option<Vec<Option<PValue>>> = None,
    pub outgoing_rps: Option<Vec<Option<StatOutReps>>> = None,
    pub outgoing_cps: Option<Vec<Option<PValue>>> = None,
    // Fit resources
    pub cpu: Option<Vec<StatResource>> = None,
    pub powergrid: Option<Vec<StatResource>> = None,
    pub calibration: Option<Vec<StatResource>> = None,
    pub drone_bay_volume: Option<Vec<StatResource>> = None,
    pub drone_bandwidth: Option<Vec<StatResource>> = None,
    pub fighter_bay_volume: Option<Vec<StatResource>> = None,
}
