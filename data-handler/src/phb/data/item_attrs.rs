use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(in crate::phb) attrs: Vec<PItemAttrData>,
}
impl FsdMerge<rc::ed::EItemAttr> for PItemAttrs {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| rc::ed::EItemAttr {
                item_id: rc::ed::EItemId::from_i32(id),
                attr_id: rc::ed::EAttrId::from_i32(v.attr_id),
                value: rc::ed::EFloat::from_f64(v.value),
            })
            .collect()
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemAttrData {
    #[serde(rename = "attributeID")]
    pub(in crate::phb) attr_id: i32,
    pub(in crate::phb) value: f64,
}
