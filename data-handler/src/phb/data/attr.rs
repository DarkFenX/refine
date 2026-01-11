use serde::Deserialize;

use crate::phb::{
    fsd::{FsdId, FsdMerge},
    serde_custom::bool_from_int,
};

#[derive(Deserialize)]
pub(in crate::phb) struct PAttr {
    #[serde(deserialize_with = "bool_from_int")]
    pub(in crate::phb) stackable: bool,
    #[serde(rename = "highIsGood", deserialize_with = "bool_from_int")]
    pub(in crate::phb) high_is_good: bool,
    #[serde(rename = "defaultValue")]
    pub(in crate::phb) default_value: f64,
    #[serde(rename = "minAttributeID")]
    pub(in crate::phb) min_attr_id: Option<i32>,
    #[serde(rename = "maxAttributeID")]
    pub(in crate::phb) max_attr_id: Option<i32>,
    #[serde(rename = "unitID")]
    pub(in crate::phb) unit_id: Option<i32>,
}
impl FsdMerge<rc::ed::EAttr> for PAttr {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr {
            id: rc::ed::EAttrId::from_i32(id),
            stackable: self.stackable,
            high_is_good: self.high_is_good,
            default_value: rc::ed::EFloat::from_f64(self.default_value),
            min_attr_id: self.min_attr_id.map(rc::ed::EAttrId::from_i32),
            max_attr_id: self.max_attr_id.map(rc::ed::EAttrId::from_i32),
            unit_id: self.unit_id.map(rc::ed::EAttrUnitId::from_i32),
        }]
    }
}
