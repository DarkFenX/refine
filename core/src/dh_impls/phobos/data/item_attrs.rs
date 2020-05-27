use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(in super::super) attrs: Vec<ItemAttrData>,
}
impl FsdMerge<dh::ItemAttr> for ItemAttrs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| dh::ItemAttr::new(id, v.attr_id, v.value))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemAttrData {
    #[serde(rename = "attributeID")]
    pub(in super::super) attr_id: ReeInt,
    pub(in super::super) value: ReeFloat,
}
