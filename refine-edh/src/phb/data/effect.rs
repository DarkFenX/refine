use serde::Deserialize;

use crate::{
    phb::data::{Key, KeyMergeOne, shared::int_to_bool},
    shared::data::{EffectMod, deser_effect_mods},
};

#[derive(Deserialize)]
pub(in crate::phb) struct PEffect {
    #[serde(rename = "effectCategory")]
    effect_category: i32,
    #[serde(rename = "isAssistance", deserialize_with = "int_to_bool")]
    is_assistance: bool,
    #[serde(rename = "isOffensive", deserialize_with = "int_to_bool")]
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
impl KeyMergeOne<rc::ed::EEffect> for PEffect {
    fn key_merge(self, key: Key, merged: &mut Vec<rc::ed::EEffect>) {
        merged.push(rc::ed::EEffect {
            id: rc::ed::EEffectId::from_i32(key),
            category_id: rc::ed::EEffectCatId::from_i32(self.effect_category),
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
        });
    }
}
