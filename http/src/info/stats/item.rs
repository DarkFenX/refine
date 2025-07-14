use crate::{
    info::stats::details::{HStatLayerEhp, HStatLayerHp, HStatLayerResist, HStatLayerRps, HStatTank},
    util::TriStateField,
};

#[derive(serde::Serialize)]
pub(crate) struct HItemStats {
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) agility: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) align_time: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) speed: TriStateField<rc::AttrVal>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) hp: TriStateField<HStatTank<HStatLayerHp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) ehp: TriStateField<Vec<Option<HStatTank<HStatLayerEhp>>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) wc_ehp: TriStateField<HStatTank<HStatLayerEhp>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rps: TriStateField<Vec<HStatTank<HStatLayerRps>>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) resists: TriStateField<HStatTank<HStatLayerResist>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rr_shield: TriStateField<Vec<rc::AttrVal>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rr_armor: TriStateField<Vec<rc::AttrVal>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rr_hull: TriStateField<Vec<rc::AttrVal>>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    pub(crate) rr_capacitor: TriStateField<Vec<rc::AttrVal>>,
}
impl HItemStats {
    pub fn new() -> Self {
        Self {
            agility: TriStateField::default(),
            align_time: TriStateField::default(),
            speed: TriStateField::default(),
            hp: TriStateField::default(),
            ehp: TriStateField::default(),
            wc_ehp: TriStateField::default(),
            rps: TriStateField::default(),
            resists: TriStateField::default(),
            rr_shield: TriStateField::default(),
            rr_armor: TriStateField::default(),
            rr_hull: TriStateField::default(),
            rr_capacitor: TriStateField::default(),
        }
    }
}
