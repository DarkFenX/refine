use serde::Deserialize;

use crate::{
    sde::data::ExtractOne,
    shared::data::{EffectMod, deser_effect_mods},
};

#[derive(Deserialize)]
pub(in crate::sde) struct SEffect {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "effectCategoryID")]
    effect_category_id: i32,
    #[serde(rename = "isAssistance")]
    is_assistance: bool,
    #[serde(rename = "isOffensive")]
    is_offensive: bool,
    #[serde(rename = "dischargeAttributeID")]
    discharge_attribute_id: Option<i32>,
    #[serde(rename = "durationAttributeID")]
    duration_attribute_id: Option<i32>,
    #[serde(rename = "rangeAttributeID")]
    range_attribute_id: Option<i32>,
    #[serde(rename = "falloffAttributeID")]
    falloff_attribute_id: Option<i32>,
    #[serde(rename = "trackingSpeedAttributeID")]
    tracking_attribute_id: Option<i32>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    fitting_usage_chance_attribute_id: Option<i32>,
    #[serde(rename = "resistanceAttributeID")]
    resistance_attribute_id: Option<i32>,
    #[serde(rename = "modifierInfo", default, deserialize_with = "deser_effect_mods")]
    modifier_info: Vec<EffectMod>,
}
impl ExtractOne<rc::ed::EEffect> for SEffect {
    fn extract(self) -> Vec<rc::ed::EEffect> {
        vec![rc::ed::EEffect {
            id: rc::ed::EEffectId::from_i32(self.id),
            category_id: rc::ed::EEffectCatId::from_i32(self.effect_category_id),
            is_assistance: self.is_assistance,
            is_offensive: self.is_offensive,
            discharge_attr_id: self.discharge_attribute_id.map(rc::ed::EAttrId::from_i32),
            duration_attr_id: self.duration_attribute_id.map(rc::ed::EAttrId::from_i32),
            range_attr_id: self.range_attribute_id.map(rc::ed::EAttrId::from_i32),
            falloff_attr_id: self.falloff_attribute_id.map(rc::ed::EAttrId::from_i32),
            tracking_attr_id: self.tracking_attribute_id.map(rc::ed::EAttrId::from_i32),
            usage_chance_attr_id: self.fitting_usage_chance_attribute_id.map(rc::ed::EAttrId::from_i32),
            resist_attr_id: self.resistance_attribute_id.map(rc::ed::EAttrId::from_i32),
            mods: self
                .modifier_info
                .into_iter()
                .map(EffectMod::into_e_effect_mod)
                .collect(),
        }]
    }
}
