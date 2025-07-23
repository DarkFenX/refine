use crate::phb::{
    fsd::{FsdId, FsdMerge},
    serde_custom::bool_from_int,
};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PAttr {
    #[serde(deserialize_with = "bool_from_int")]
    pub(in crate::phb) stackable: bool,
    #[serde(rename = "highIsGood", deserialize_with = "bool_from_int")]
    pub(in crate::phb) high_is_good: bool,
    #[serde(rename = "defaultValue")]
    pub(in crate::phb) default_value: rc::ed::EAttrVal,
    #[serde(rename = "minAttributeID")]
    pub(in crate::phb) min_attr_id: Option<rc::ed::EAttrId>,
    #[serde(rename = "maxAttributeID")]
    pub(in crate::phb) max_attr_id: Option<rc::ed::EAttrId>,
    #[serde(rename = "unitID")]
    pub(in crate::phb) unit_id: Option<rc::ed::EAttrUnitId>,
}
impl FsdMerge<rc::ed::EAttr> for PAttr {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr {
            id,
            stackable: self.stackable,
            high_is_good: self.high_is_good,
            default_value: self.default_value,
            min_attr_id: self.min_attr_id,
            max_attr_id: self.max_attr_id,
            unit_id: self.unit_id,
        }]
    }
}
