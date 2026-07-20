use crate::{
    PValue,
    stats::{StatDmg, StatMining, StatOutReps},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FleetStats {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dmg: Vec<Option<StatDmg>> = Vec::new(),
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mps: Vec<StatMining> = Vec::new(),
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outgoing_nps: Vec<Option<PValue>> = Vec::new(),
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outgoing_rps: Vec<Option<StatOutReps>> = Vec::new(),
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub outgoing_cps: Vec<Option<PValue>> = Vec::new(),
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mass: Vec<PValue> = Vec::new(),
}
