use crate::{
    defs::ReeInt,
    edh_impls::phobos::{data::aux::into_vec, fsd::FsdMerge},
    edt,
};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Buff {
    #[serde(rename = "aggregateMode")]
    pub(in super::super) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(in super::super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(in super::super) item_mods: Vec<BuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(in super::super) loc_mods: Vec<BuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(in super::super) locgroup_mods: Vec<BuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(in super::super) locsrq_mods: Vec<BuffLRSM>,
}
impl FsdMerge<edt::Buff> for Buff {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::Buff> {
        vec![edt::Buff::new(
            id,
            self.aggregate_mode,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
}
impl Into<edt::BuffIM> for BuffIM {
    fn into(self) -> edt::BuffIM {
        edt::BuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
}
impl Into<edt::BuffLM> for BuffLM {
    fn into(self) -> edt::BuffLM {
        edt::BuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(in super::super) group_id: ReeInt,
}
impl Into<edt::BuffLGM> for BuffLGM {
    fn into(self) -> edt::BuffLGM {
        edt::BuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(in super::super) skill_id: ReeInt,
}
impl Into<edt::BuffLRSM> for BuffLRSM {
    fn into(self) -> edt::BuffLRSM {
        edt::BuffLRSM::new(self.attr_id, self.skill_id)
    }
}
