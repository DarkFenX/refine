use crate::{
    defs::{ReeFloat, ReeInt},
    edh_impls::phobos::fsd::FsdMerge,
    edt,
};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Attr {
    pub(in super::super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(in super::super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(in super::super) default_value: Option<ReeFloat>,
    #[serde(rename = "maxAttributeID")]
    pub(in super::super) max_attr_id: Option<ReeInt>,
    #[serde(rename = "unitID")]
    pub(in super::super) unit_id: Option<ReeInt>,
}
impl FsdMerge<edt::Attr> for Attr {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::Attr> {
        vec![edt::Attr::new(
            id,
            self.stackable != 0,
            self.high_is_good != 0,
            self.default_value,
            self.max_attr_id,
            self.unit_id,
        )]
    }
}
