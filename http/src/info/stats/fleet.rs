use crate::info::stats::details::{HStatDmg, HStatTank};

#[derive(serde::Serialize)]
pub(crate) struct HFleetStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dps: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volley: Option<Vec<Option<HStatDmg>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote_rps: Option<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote_cps: Option<rc::AttrVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) remote_nps: Option<Vec<rc::AttrVal>>,
}
impl HFleetStats {
    pub fn new() -> Self {
        Self {
            dps: Option::default(),
            volley: Option::default(),
            remote_rps: Option::default(),
            remote_cps: Option::default(),
            remote_nps: Option::default(),
        }
    }
}
