use serde::Deserialize;

use crate::phb::{
    parsing::{Key, KeyMergeTwo},
    serde_custom::bool_from_int,
};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemDogma {
    #[serde(rename = "dogmaAttributes", default)]
    pub(in crate::phb) attrs: Vec<PItemAttrData>,
    #[serde(rename = "dogmaEffects", default)]
    pub(in crate::phb) effects: Vec<PItemEffectData>,
}
impl KeyMergeTwo<rc::ed::EItemAttr, rc::ed::EItemEffect> for PItemDogma {
    fn key_merge(self, key: Key) -> (Vec<rc::ed::EItemAttr>, Vec<rc::ed::EItemEffect>) {
        let item_attrs = self
            .attrs
            .into_iter()
            .map(|v| rc::ed::EItemAttr {
                item_id: rc::ed::EItemId::from_i32(key),
                attr_id: rc::ed::EAttrId::from_i32(v.attr_id),
                value: rc::ed::EFloat::from_f64(v.value),
            })
            .collect();
        let item_effects = self
            .effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect {
                item_id: rc::ed::EItemId::from_i32(key),
                effect_id: rc::ed::EEffectId::from_i32(v.effect_id),
                is_default: v.is_default,
            })
            .collect();
        (item_attrs, item_effects)
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemAttrData {
    #[serde(rename = "attributeID")]
    pub(in crate::phb) attr_id: i32,
    pub(in crate::phb) value: f64,
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemEffectData {
    #[serde(rename = "effectID")]
    pub(in crate::phb) effect_id: i32,
    #[serde(rename = "isDefault", deserialize_with = "bool_from_int")]
    pub(in crate::phb) is_default: bool,
}
