use crate::{fsd::FsdMerge, util::into_vec};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Buff {
    #[serde(rename = "aggregateMode")]
    pub(crate) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(crate) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(crate) item_mods: Vec<BuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(crate) loc_mods: Vec<BuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(crate) locgroup_mods: Vec<BuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(crate) locsrq_mods: Vec<BuffLRSM>,
}
impl FsdMerge<rc::edt::EBuff> for Buff {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::EBuff> {
        vec![rc::edt::EBuff::new(
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
pub(crate) struct BuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
}
impl Into<rc::edt::EBuffIM> for BuffIM {
    fn into(self) -> rc::edt::EBuffIM {
        rc::edt::EBuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
}
impl Into<rc::edt::EBuffLM> for BuffLM {
    fn into(self) -> rc::edt::EBuffLM {
        rc::edt::EBuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "groupID")]
    pub(crate) group_id: rc::ReeInt,
}
impl Into<rc::edt::EBuffLGM> for BuffLGM {
    fn into(self) -> rc::edt::EBuffLGM {
        rc::edt::EBuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "skillID")]
    pub(crate) skill_id: rc::ReeInt,
}
impl Into<rc::edt::EBuffLRSM> for BuffLRSM {
    fn into(self) -> rc::edt::EBuffLRSM {
        rc::edt::EBuffLRSM::new(self.attr_id, self.skill_id)
    }
}
