use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PAttr {
    pub(in crate::phb) stackable: rc::ReeInt,
    #[serde(rename = "highIsGood")]
    pub(in crate::phb) high_is_good: rc::ReeInt,
    #[serde(rename = "defaultValue")]
    pub(in crate::phb) default_value: Option<rc::ReeFloat>,
    #[serde(rename = "maxAttributeID")]
    pub(in crate::phb) max_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "unitID")]
    pub(in crate::phb) unit_id: Option<rc::ReeInt>,
}
impl FsdMerge<rc::ed::EAttr> for PAttr {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EAttr> {
        vec![rc::ed::EAttr::new(
            id,
            self.stackable != 0,
            self.high_is_good != 0,
            self.default_value,
            self.max_attr_id,
            self.unit_id,
        )]
    }
}
