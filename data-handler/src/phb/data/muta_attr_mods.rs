use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(in crate::phb) attrs: HashMap<i32, PMutaAttrModRange>,
}
impl FsdMerge<rc::ed::EMutaAttrMod> for PMutaAttrMods {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EMutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttrMod {
                muta_id: rc::ed::EItemId::from_i32(id),
                attr_id: rc::ed::EAttrId::from_i32(attr_id),
                min_attr_mult: rc::ed::EFloat::from_f64(range.min),
                max_attr_mult: rc::ed::EFloat::from_f64(range.max),
            })
            .collect()
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrModRange {
    pub(in crate::phb) min: f64,
    pub(in crate::phb) max: f64,
}
