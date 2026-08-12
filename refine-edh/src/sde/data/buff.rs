use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SBuff {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "aggregateMode")]
    aggregate_mode: String,
    #[serde(rename = "operationName")]
    operation_name: String,
    #[serde(rename = "itemModifiers", default)]
    item_modifiers: Vec<SBuffIM>,
    #[serde(rename = "locationModifiers", default)]
    location_modifiers: Vec<SBuffLM>,
    #[serde(rename = "locationGroupModifiers", default)]
    location_group_modifiers: Vec<SBuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers", default)]
    location_required_skill_modifiers: Vec<SBuffLRSM>,
}
impl ExtractOne<rc::ed::EBuff> for SBuff {
    fn extract(self) -> Vec<rc::ed::EBuff> {
        vec![rc::ed::EBuff {
            id: rc::ed::EBuffId::from_i32(self.id),
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
struct SBuffIM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl SBuffIM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffIM {
        rc::ed::EBuffIM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
struct SBuffLM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl SBuffLM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLM {
        rc::ed::EBuffLM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
struct SBuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "groupID")]
    group_id: i32,
}
impl SBuffLGM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLGM {
        rc::ed::EBuffLGM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
        }
    }
}

#[derive(Deserialize)]
struct SBuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "skillID")]
    skill_id: i32,
}
impl SBuffLRSM {
    fn into_e_buff_mod(self) -> rc::ed::EBuffLRSM {
        rc::ed::EBuffLRSM {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            skill_id: rc::ed::EItemId::from_i32(self.skill_id),
        }
    }
}
