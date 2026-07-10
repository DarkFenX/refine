use serde::Deserialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::phb::parsing::{Key, KeyMergeTwo};

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PMuta {
    #[serde(rename = "inputOutputMapping")]
    pub(in crate::phb) item_maps: Vec<PMutaItemMap>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(rename = "attributeIDs")]
    pub(in crate::phb) attrs: Vec<(i32, PMutaAttrModRange)>,
}
impl KeyMergeTwo<rc::ed::EMutaItemConv, rc::ed::EMutaAttrMod> for PMuta {
    fn key_merge(self, key: Key) -> (Vec<rc::ed::EMutaItemConv>, Vec<rc::ed::EMutaAttrMod>) {
        let mut muta_items = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                muta_items.push(rc::ed::EMutaItemConv {
                    muta_id: rc::ed::EItemId::from_i32(key),
                    in_item_id: rc::ed::EItemId::from_i32(applicable_type),
                    out_item_id: rc::ed::EItemId::from_i32(item_map.result_item_id),
                })
            }
        }
        let muta_attrs = self
            .attrs
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttrMod {
                muta_id: rc::ed::EItemId::from_i32(key),
                attr_id: rc::ed::EAttrId::from_i32(attr_id),
                min_attr_mult: rc::ed::EFloat::from_f64(range.min),
                max_attr_mult: rc::ed::EFloat::from_f64(range.max),
            })
            .collect();
        (muta_items, muta_attrs)
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PMutaAttrModRange {
    pub(in crate::phb) min: f64,
    pub(in crate::phb) max: f64,
}

#[derive(Deserialize)]
pub(in crate::phb) struct PMutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(in crate::phb) applicable_item_ids: Vec<i32>,
    #[serde(rename = "resultingType")]
    pub(in crate::phb) result_item_id: i32,
}
