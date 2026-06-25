use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::phb::parsing::{Key, KeyMerge};

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PMutaAttrMods {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "attributeIDs")]
    pub(in crate::phb) attrs: Vec<(i32, PMutaAttrModRange)>,
}
impl KeyMerge<rc::ed::EMutaAttrMod> for PMutaAttrMods {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EMutaAttrMod> {
        self.attrs
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttrMod {
                muta_id: rc::ed::EItemId::from_i32(key),
                attr_id: rc::ed::EAttrId::from_i32(attr_id),
                min_attr_mult: rc::ed::EFloat::from_f64(range.min),
                max_attr_mult: rc::ed::EFloat::from_f64(range.max),
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PMutaAttrModRange {
    pub(in crate::phb) min: f64,
    pub(in crate::phb) max: f64,
}
