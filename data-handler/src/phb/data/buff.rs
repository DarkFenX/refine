use serde::Deserialize;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Deserialize)]
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
            item_mods: self
                .item_mods
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            loc_mods: self
                .loc_mods
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            locgroup_mods: self
                .locgroup_mods
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            locsrq_mods: self
                .locsrq_mods
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
        }]
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
}
impl PBuffIM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffIM {
        rc::ed::EBuffIM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
}
impl PBuffLM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLM {
        rc::ed::EBuffLM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: i32,
}
impl PBuffLGM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLGM {
        rc::ed::EBuffLGM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
        }
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in crate::phb) attr_id: i32,
    #[serde(rename = "skillID")]
    pub(in crate::phb) skill_id: i32,
}
impl PBuffLRSM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLRSM {
        rc::ed::EBuffLRSM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            skill_id: rc::ed::EItemId::from_i32(self.skill_id),
        }
    }
}
