use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(in crate::phb) attrs: HashMap<rc::ed::EAttrId, PMutaAttrModRange>,
}
impl FsdMerge<rc::ed::EMutaAttrMod> for PMutaAttrMods {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EMutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttrMod {
                muta_id: id,
                attr_id,
                min_attr_mult: range.min,
                max_attr_mult: range.max,
            })
            .collect()
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrModRange {
    pub(in crate::phb) min: rc::ed::EAttrVal,
    pub(in crate::phb) max: rc::ed::EAttrVal,
}
