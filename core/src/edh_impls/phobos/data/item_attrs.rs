use crate::{
    defs::{ReeFloat, ReeInt},
    edh_impls::phobos::fsd::FsdMerge,
    edt,
};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(in super::super) attrs: Vec<ItemAttrData>,
}
impl FsdMerge<edt::ItemAttr> for ItemAttrs {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::ItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| edt::ItemAttr::new(id, v.attr_id, v.value))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemAttrData {
    #[serde(rename = "attributeID")]
    pub(in super::super) attr_id: ReeInt,
    pub(in super::super) value: ReeFloat,
}
