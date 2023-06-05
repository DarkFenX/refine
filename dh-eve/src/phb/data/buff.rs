use crate::{phb::fsd::FsdMerge, util::into_vec};

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
impl FsdMerge<rc::ed::EBuff> for Buff {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EBuff> {
        vec![rc::ed::EBuff::new(
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
impl Into<rc::ed::EBuffIM> for BuffIM {
    fn into(self) -> rc::ed::EBuffIM {
        rc::ed::EBuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLM> for BuffLM {
    fn into(self) -> rc::ed::EBuffLM {
        rc::ed::EBuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "groupID")]
    pub(crate) group_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLGM> for BuffLGM {
    fn into(self) -> rc::ed::EBuffLGM {
        rc::ed::EBuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "skillID")]
    pub(crate) skill_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLRSM> for BuffLRSM {
    fn into(self) -> rc::ed::EBuffLRSM {
        rc::ed::EBuffLRSM::new(self.attr_id, self.skill_id)
    }
}
