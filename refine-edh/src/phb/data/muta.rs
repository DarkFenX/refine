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
    attribute_ids: Vec<(i32, PMutaAttrRange)>,
}
impl KeyMergeTwo<rc::ed::EMutaItem, rc::ed::EMutaAttr> for PMuta {
    fn key_merge(self, key: Key) -> (Vec<rc::ed::EMutaItem>, Vec<rc::ed::EMutaAttr>) {
        let muta_items = self
            .input_output_mapping
            .into_iter()
            .flat_map(|item_map| {
                item_map
                    .applicable_types
                    .into_iter()
                    .map(move |in_item_id| rc::ed::EMutaItem {
                        muta_id: rc::ed::EItemId::from_i32(key),
                        in_item_id: rc::ed::EItemId::from_i32(in_item_id),
                        out_item_id: rc::ed::EItemId::from_i32(item_map.resulting_type),
                    })
            })
            .collect();
        let muta_attrs = self
            .attribute_ids
            .into_iter()
            .map(|(attr_id, range)| rc::ed::EMutaAttr {
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
struct PMutaItemMap {
    #[serde(rename = "applicableTypes")]
    applicable_types: Vec<i32>,
    #[serde(rename = "resultingType")]
    resulting_type: i32,
}

#[derive(Deserialize)]
struct PMutaAttrRange {
    min: f64,
    max: f64,
}
