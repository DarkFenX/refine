use serde::Deserialize;

use crate::sde::data::ExtractTwo;

#[derive(Deserialize)]
pub(in crate::sde) struct SItemDogma {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(rename = "dogmaAttributes", default)]
    dogma_attributes: Vec<SItemAttrData>,
    #[serde(rename = "dogmaEffects", default)]
    dogma_effects: Vec<SItemEffectData>,
}
impl ExtractTwo<rc::ed::EItemAttr, rc::ed::EItemEffect> for SItemDogma {
    fn extract(self) -> (Vec<rc::ed::EItemAttr>, Vec<rc::ed::EItemEffect>) {
        let item_attrs = self
            .dogma_attributes
            .into_iter()
            .map(|v| rc::ed::EItemAttr {
                item_id: rc::ed::EItemId::from_i32(self.item_id),
                attr_id: rc::ed::EAttrId::from_i32(v.attribute_id),
                value: rc::ed::EFloat::from_f64(v.value),
            })
            .collect();
        let item_effects = self
            .dogma_effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect {
                item_id: rc::ed::EItemId::from_i32(self.item_id),
                effect_id: rc::ed::EEffectId::from_i32(v.effect_id),
                is_default: v.is_default,
            })
            .collect();
        (item_attrs, item_effects)
    }
}

#[derive(Deserialize)]
struct SItemAttrData {
    #[serde(rename = "attributeID")]
    attribute_id: i32,
    value: f64,
}

#[derive(Deserialize)]
struct SItemEffectData {
    #[serde(rename = "effectID")]
    effect_id: i32,
    #[serde(rename = "isDefault")]
    is_default: bool,
}
