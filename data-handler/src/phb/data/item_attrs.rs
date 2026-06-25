use serde::Deserialize;

use crate::phb::parsing::{Key, KeyMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemAttrs {
    #[serde(rename = "dogmaAttributes", default)]
    pub(in crate::phb) attrs: Vec<PItemAttrData>,
}
impl KeyMerge<rc::ed::EItemAttr> for PItemAttrs {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemAttr> {
        self.attrs
            .into_iter()
            .map(|v| rc::ed::EItemAttr {
                item_id: rc::ed::EItemId::from_i32(key),
                attr_id: rc::ed::EAttrId::from_i32(v.attr_id),
                value: rc::ed::EFloat::from_f64(v.value),
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemAttrData {
    #[serde(rename = "attributeID")]
    pub(in crate::phb) attr_id: i32,
    pub(in crate::phb) value: f64,
}
