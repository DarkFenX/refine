use crate::phb::{
    fsd::{FsdId, FsdMerge},
    serde_custom::bool_from_int,
};

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PAttr {
    #[serde(deserialize_with = "bool_from_int")]
    pub(in crate::phb) stackable: bool,
    #[serde(rename = "highIsGood", deserialize_with = "bool_from_int")]
    pub(in crate::phb) high_is_good: bool,
    #[serde(rename = "defaultValue")]
    pub(in crate::phb) default_value: Option<rc::AttrVal>,
    #[serde(rename = "maxAttributeID")]
    pub(in crate::phb) max_attr_id: Option<rc::EAttrId>,
    #[serde(rename = "unitID")]
    pub(in crate::phb) unit_id: Option<rc::EAttrUnitId>,
}
impl FsdMerge<rc::ed::EAttr> for PAttr {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr::new(
            id,
            self.stackable,
            self.high_is_good,
            self.default_value,
            self.max_attr_id,
            self.unit_id,
        )]
    }
}
