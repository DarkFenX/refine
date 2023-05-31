use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Attr {
    pub(crate) stackable: rc::ReeInt,
    #[serde(rename = "highIsGood")]
    pub(crate) high_is_good: rc::ReeInt,
    #[serde(rename = "defaultValue")]
    pub(crate) default_value: Option<rc::ReeFloat>,
    #[serde(rename = "maxAttributeID")]
    pub(crate) max_attr_id: Option<rc::ReeInt>,
    #[serde(rename = "unitID")]
    pub(crate) unit_id: Option<rc::ReeInt>,
}
impl FsdMerge<rc::edt::Attr> for Attr {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::Attr> {
        vec![rc::edt::Attr::new(
            id,
            self.stackable != 0,
            self.high_is_good != 0,
            self.default_value,
            self.max_attr_id,
            self.unit_id,
        )]
    }
}
