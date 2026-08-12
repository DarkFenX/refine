use serde::Deserialize;

use crate::{
    phb::data::{Key, KeyMergeOne},
    shared::data::{BuffIm, BuffLgm, BuffLm, BuffLrsm},
};

#[derive(Deserialize)]
pub(in crate::phb) struct PBuff {
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
impl KeyMergeOne<rc::ed::EBuff> for PBuff {
    fn key_merge(self, key: Key, merged: &mut Vec<rc::ed::EBuff>) {
        merged.push(rc::ed::EBuff {
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
        });
    }
}
