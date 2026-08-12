use serde::Deserialize;

use crate::{
    phb::data::{Key, KeyMergeOne},
    util::bool_from_int,
};

#[derive(Deserialize)]
pub(in crate::phb) struct PAttr {
    #[serde(deserialize_with = "bool_from_int")]
    stackable: bool,
    #[serde(rename = "highIsGood", deserialize_with = "bool_from_int")]
    high_is_good: bool,
    #[serde(rename = "defaultValue")]
    default_value: f64,
    #[serde(rename = "minAttributeID")]
    min_attribute_id: Option<i32>,
    #[serde(rename = "maxAttributeID")]
    max_attribute_id: Option<i32>,
    #[serde(rename = "unitID")]
    unit_id: Option<i32>,
}
impl KeyMergeOne<rc::ed::EAttr> for PAttr {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr {
            id: rc::ed::EAttrId::from_i32(key),
            stackable: self.stackable,
            high_is_good: self.high_is_good,
            default_value: rc::ed::EFloat::from_f64(self.default_value),
            min_attr_id: self.min_attribute_id.map(rc::ed::EAttrId::from_i32),
            max_attr_id: self.max_attribute_id.map(rc::ed::EAttrId::from_i32),
            unit_id: self.unit_id.map(rc::ed::EAttrUnitId::from_i32),
        }]
    }
}
