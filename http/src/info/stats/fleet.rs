use serde::Serialize;

use crate::info::stats::details::{HStatDmg, HStatMining, HStatOutReps};

#[derive(Serialize)]
pub(crate) struct HFleetStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dmg: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mps: Option<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_nps: Option<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_rps: Option<Vec<Option<HStatOutReps>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_cps: Option<Vec<Option<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mass: Option<Vec<f64>>,
}
impl HFleetStats {
    pub(crate) fn new() -> Self {
        Self {
            dmg: Option::default(),
            mps: Option::default(),
            outgoing_rps: Option::default(),
            outgoing_cps: Option::default(),
            outgoing_nps: Option::default(),
            mass: Option::default(),
        }
    }
}
