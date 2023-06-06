use crate::{phb::fsd::FsdMerge, util::into_vec};

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PBuff {
    #[serde(rename = "aggregateMode")]
    pub(in crate::phb) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(in crate::phb) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(in crate::phb) item_mods: Vec<PBuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(in crate::phb) loc_mods: Vec<PBuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(in crate::phb) locgroup_mods: Vec<PBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(in crate::phb) locsrq_mods: Vec<PBuffLRSM>,
}
impl FsdMerge<rc::ed::EBuff> for PBuff {
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
pub(in crate::phb) struct PBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffIM> for PBuffIM {
    fn into(self) -> rc::ed::EBuffIM {
        rc::ed::EBuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLM> for PBuffLM {
    fn into(self) -> rc::ed::EBuffLM {
        rc::ed::EBuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: rc::ReeInt,
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLGM> for PBuffLGM {
    fn into(self) -> rc::ed::EBuffLGM {
        rc::ed::EBuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: rc::ReeInt,
    #[serde(rename = "skillID")]
    pub(in crate::phb) skill_id: rc::ReeInt,
}
impl Into<rc::ed::EBuffLRSM> for PBuffLRSM {
    fn into(self) -> rc::ed::EBuffLRSM {
        rc::ed::EBuffLRSM::new(self.attr_id, self.skill_id)
    }
}
