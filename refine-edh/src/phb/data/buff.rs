use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PBuff {
    #[serde(rename = "aggregateMode")]
    aggregate_mode: String,
    #[serde(rename = "operationName")]
    operation_name: String,
    #[serde(rename = "itemModifiers", default)]
    item_modifiers: Vec<PBuffIM>,
    #[serde(rename = "locationModifiers", default)]
    location_modifiers: Vec<PBuffLM>,
    #[serde(rename = "locationGroupModifiers", default)]
    location_group_modifiers: Vec<PBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers", default)]
    location_required_skill_modifiers: Vec<PBuffLRSM>,
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
struct PBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl PBuffIM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffIM {
        rc::ed::EBuffIM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
struct PBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl PBuffLM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLM {
        rc::ed::EBuffLM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
struct PBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "groupID")]
    group_id: i32,
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
struct PBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "skillID")]
    skill_id: i32,
}
impl PBuffLRSM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLRSM {
        rc::ed::EBuffLRSM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            skill_id: rc::ed::EItemId::from_i32(self.skill_id),
        }
    }
}
