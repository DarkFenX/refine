use crate::info::stats::details::{HStatDmg, HStatMining, HStatTank};

#[derive(serde::Serialize)]
pub(crate) struct HFleetStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dps: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volley: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mps: Option<Vec<HStatMining>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_rps: Option<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_cps: Option<rc::AttrVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outgoing_nps: Option<Vec<Option<rc::AttrVal>>>,
}
impl HFleetStats {
    pub(crate) fn new() -> Self {
        Self {
            dps: Option::default(),
            volley: Option::default(),
            mps: Option::default(),
            outgoing_rps: Option::default(),
            outgoing_cps: Option::default(),
            outgoing_nps: Option::default(),
        }
    }
}
