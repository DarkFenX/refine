use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeTwo, shared::int_to_bool};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemDogma {
    #[serde(rename = "dogmaAttributes", default)]
    dogma_attributes: Vec<PItemAttrData>,
    #[serde(rename = "dogmaEffects", default)]
    dogma_effects: Vec<PItemEffectData>,
}
impl KeyMergeTwo<rc::ed::EItemAttr, rc::ed::EItemEffect> for PItemDogma {
    fn key_merge(self, key: Key, merged1: &mut Vec<rc::ed::EItemAttr>, merged2: &mut Vec<rc::ed::EItemEffect>) {
        merged1.extend(self.dogma_attributes.into_iter().map(|v| rc::ed::EItemAttr {
            item_id: rc::ed::EItemId::from_i32(key),
            attr_id: rc::ed::EAttrId::from_i32(v.attribute_id),
            value: rc::ed::EFloat::from_f64(v.value),
        }));
        merged2.extend(self.dogma_effects.into_iter().map(|v| rc::ed::EItemEffect {
            item_id: rc::ed::EItemId::from_i32(key),
            effect_id: rc::ed::EEffectId::from_i32(v.effect_id),
            is_default: v.is_default,
        }));
    }
}

#[derive(Deserialize)]
struct PItemAttrData {
    #[serde(rename = "attributeID")]
    attribute_id: i32,
    value: f64,
}

#[derive(Deserialize)]
struct PItemEffectData {
    #[serde(rename = "effectID")]
    effect_id: i32,
    #[serde(rename = "isDefault", deserialize_with = "int_to_bool")]
    is_default: bool,
}
