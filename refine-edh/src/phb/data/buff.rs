use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PBuff {
    #[serde(rename = "aggregateMode")]
    pub(in crate::phb) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(in crate::phb) operation_name: String,
    #[serde(rename = "itemModifiers", default)]
    pub(in crate::phb) item_modifiers: Vec<PBuffIM>,
    #[serde(rename = "locationModifiers", default)]
    pub(in crate::phb) location_modifiers: Vec<PBuffLM>,
    #[serde(rename = "locationGroupModifiers", default)]
    pub(in crate::phb) location_group_modifiers: Vec<PBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers", default)]
    pub(in crate::phb) location_required_skill_modifiers: Vec<PBuffLRSM>,
}
impl KeyMergeOne<rc::ed::EBuff> for PBuff {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EBuff> {
        vec![rc::ed::EBuff {
            id: rc::ed::EBuffId::from_i32(key),
            aggregate_mode: self.aggregate_mode,
            operation: self.operation_name,
            item_mods: self
                .item_modifiers
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            loc_mods: self
                .location_modifiers
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            locgroup_mods: self
                .location_group_modifiers
                .into_iter()
                .map(|p_buff_mod| p_buff_mod.into_e_buff_mod())
                .collect(),
            locsrq_mods: self
                .location_required_skill_modifiers
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
