use serde::Deserialize;
use serde_with::{Map, serde_as};

use crate::phb::data::{Key, KeyMergeTwo};

#[serde_as]
#[derive(Deserialize)]
pub(in crate::phb) struct PMuta {
    #[serde(rename = "inputOutputMapping")]
    input_output_mapping: Vec<PMutaItemMap>,
    #[serde_as(as = "Map<_, _>")]
    #[serde(rename = "attributeIDs")]
    attribute_ids: Vec<(i32, PMutaAttrModRange)>,
}
impl KeyMergeTwo<rc::ed::EMutaItemConv, rc::ed::EMutaAttrMod> for PMuta {
    fn key_merge(self, key: Key) -> (Vec<rc::ed::EMutaItemConv>, Vec<rc::ed::EMutaAttrMod>) {
        let mut muta_items = Vec::new();
        for item_map in self.input_output_mapping {
            for applicable_type in item_map.applicable_types {
                muta_items.push(rc::ed::EMutaItemConv {
                    muta_id: rc::ed::EItemId::from_i32(key),
                    in_item_id: rc::ed::EItemId::from_i32(applicable_type),
                    out_item_id: rc::ed::EItemId::from_i32(item_map.resulting_type),
                })
            }
        }
        let muta_attrs = self
            .attribute_ids
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
struct PMutaAttrModRange {
    min: f64,
    max: f64,
}

#[derive(Deserialize)]
struct PMutaItemMap {
    #[serde(rename = "applicableTypes")]
    applicable_types: Vec<i32>,
    #[serde(rename = "resultingType")]
    resulting_type: i32,
}
