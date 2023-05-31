use std::collections::HashMap;

use crate::{
    defs::{ReeFloat, ReeInt},
    edh_impls::phobos::fsd::FsdMerge,
    edt,
};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct MutaAttrMods {
    #[serde(rename = "attributeIDs")]
    pub(in super::super) attrs: HashMap<ReeInt, MutaAttrModRange>,
}
impl FsdMerge<edt::MutaAttrMod> for MutaAttrMods {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::MutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| edt::MutaAttrMod::new(id, attr_id, range.min, range.max))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct MutaAttrModRange {
    pub(in super::super) min: ReeFloat,
    pub(in super::super) max: ReeFloat,
}
