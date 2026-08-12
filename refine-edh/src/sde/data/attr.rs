use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SAttr {
    #[serde(rename = "_key")]
    id: i32,
    stackable: bool,
    #[serde(rename = "highIsGood")]
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
impl ExtractOne<rc::ed::EAttr> for SAttr {
    fn extract(self) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr {
            id: rc::ed::EAttrId::from_i32(self.id),
            stackable: self.stackable,
            high_is_good: self.high_is_good,
            default_value: rc::ed::EFloat::from_f64(self.default_value),
            min_attr_id: self.min_attribute_id.map(rc::ed::EAttrId::from_i32),
            max_attr_id: self.max_attribute_id.map(rc::ed::EAttrId::from_i32),
            unit_id: self.unit_id.map(rc::ed::EAttrUnitId::from_i32),
        }]
    }
}
