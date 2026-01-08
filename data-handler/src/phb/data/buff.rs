use crate::{
    phb::fsd::{FsdId, FsdMerge},
    util::into_vec,
};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PBuff {
    #[serde(rename = "aggregateMode")]
    pub(in crate::phb) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(in crate::phb) operation: String,
    #[serde(rename = "itemModifiers", default)]
    pub(in crate::phb) item_mods: Vec<PBuffIM>,
    #[serde(rename = "locationModifiers", default)]
    pub(in crate::phb) loc_mods: Vec<PBuffLM>,
    #[serde(rename = "locationGroupModifiers", default)]
    pub(in crate::phb) locgroup_mods: Vec<PBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers", default)]
    pub(in crate::phb) locsrq_mods: Vec<PBuffLRSM>,
}
impl FsdMerge<rc::ed::EBuff> for PBuff {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EBuff> {
        vec![rc::ed::EBuff {
            id: rc::ed::EBuffId::from_i32(id),
            aggregate_mode: self.aggregate_mode,
            operation: self.operation,
            item_mods: into_vec(self.item_mods),
            loc_mods: into_vec(self.loc_mods),
            locgroup_mods: into_vec(self.locgroup_mods),
            locsrq_mods: into_vec(self.locsrq_mods),
        }]
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
}
impl From<PBuffIM> for rc::ed::EBuffIM {
    fn from(p_buff_im: PBuffIM) -> Self {
        Self {
            attr_id: rc::ed::EAttrId::from_i32(p_buff_im.attr_id),
        }
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
}
impl From<PBuffLM> for rc::ed::EBuffLM {
    fn from(p_buff_lm: PBuffLM) -> Self {
        Self {
            attr_id: rc::ed::EAttrId::from_i32(p_buff_lm.attr_id),
        }
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: i32,
}
impl From<PBuffLGM> for rc::ed::EBuffLGM {
    fn from(p_buff_lgm: PBuffLGM) -> Self {
        Self {
            attr_id: rc::ed::EAttrId::from_i32(p_buff_lgm.attr_id),
            group_id: rc::ed::EItemGrpId::from_i32(p_buff_lgm.group_id),
        }
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
    #[serde(rename = "skillID")]
    pub(in crate::phb) skill_id: i32,
}
impl From<PBuffLRSM> for rc::ed::EBuffLRSM {
    fn from(p_buff_lrsm: PBuffLRSM) -> Self {
        Self {
            attr_id: rc::ed::EAttrId::from_i32(p_buff_lrsm.attr_id),
            skill_id: rc::ed::EItemId::from_i32(p_buff_lrsm.skill_id),
        }
    }
}
