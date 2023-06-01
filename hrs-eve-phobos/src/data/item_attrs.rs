use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(crate) attrs: Vec<ItemAttrData>,
}
impl FsdMerge<rc::edt::ItemAttr> for ItemAttrs {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::ItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| rc::edt::ItemAttr::new(id, v.attr_id, v.value))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemAttrData {
    #[serde(rename = "attributeID")]
    pub(crate) attr_id: rc::ReeInt,
    pub(crate) value: rc::ReeFloat,
}
