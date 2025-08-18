use crate::{
    info::stats::details::{
        HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerHp, HStatLayerResist, HStatLayerRps, HStatTank,
    },
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HItemStats {
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) sig_radius: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) dps: TriStateField<Vec<HStatDmg>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) volley: TriStateField<Vec<HStatDmg>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatTank<HStatLayerHp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<HStatTank<Option<HStatLayerEhp>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatTank<Option<HStatLayerEhp>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatTank<HStatLayerRps>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) erps: TriStateField<Vec<HStatTank<Option<HStatLayerErps>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatTank<HStatLayerResist>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) remote_rps: TriStateField<Vec<HStatTank<rc::AttrVal>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) remote_cps: TriStateField<Vec<rc::AttrVal>>,
}
impl HItemStats {
    pub fn new() -> Self {
        Self {
            speed: TriStateField::default(),
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            sig_radius: TriStateField::default(),
            dps: TriStateField::default(),
            volley: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            erps: TriStateField::default(),
            resists: TriStateField::default(),
            remote_rps: TriStateField::default(),
            remote_cps: TriStateField::default(),
        }
    }
}
