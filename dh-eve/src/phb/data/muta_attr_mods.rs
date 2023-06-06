use std::collections::HashMap;

use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(in crate::phb) attrs: HashMap<rc::ReeInt, PMutaAttrModRange>,
}
impl FsdMerge<rc::ed::EMutaAttrMod> for PMutaAttrMods {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EMutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttrMod::new(id, attr_id, range.min, range.max))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PMutaAttrModRange {
    pub(in crate::phb) min: rc::ReeFloat,
    pub(in crate::phb) max: rc::ReeFloat,
}
