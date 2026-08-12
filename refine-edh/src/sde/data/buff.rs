use serde::Deserialize;

use crate::{
    sde::data::ExtractOne,
    shared::data::{BuffIm, BuffLgm, BuffLm, BuffLrsm},
};

#[derive(Deserialize)]
pub(in crate::sde) struct SBuff {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "aggregateMode")]
    aggregate_mode: String,
    #[serde(rename = "operationName")]
    operation_name: String,
    #[serde(rename = "itemModifiers", default)]
    item_modifiers: Vec<BuffIm>,
    #[serde(rename = "locationModifiers", default)]
    location_modifiers: Vec<BuffLm>,
    #[serde(rename = "locationGroupModifiers", default)]
    location_group_modifiers: Vec<BuffLgm>,
    #[serde(rename = "locationRequiredSkillModifiers", default)]
    location_required_skill_modifiers: Vec<BuffLrsm>,
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
