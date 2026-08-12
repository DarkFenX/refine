use serde::Deserialize;

use crate::sde::data::ExtractTwo;

#[derive(Deserialize)]
pub(in crate::sde) struct SMuta {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(rename = "inputOutputMapping")]
    input_output_mapping: Vec<SMutaItemMap>,
    #[serde(rename = "attributeIDs")]
    attribute_ids: Vec<SMutaAttrRange>,
}
impl ExtractTwo<rc::ed::EMutaItem, rc::ed::EMutaAttr> for SMuta {
    fn extract(self, extracted1: &mut Vec<rc::ed::EMutaItem>, extracted2: &mut Vec<rc::ed::EMutaAttr>) {
        extracted1.extend(self.input_output_mapping.into_iter().flat_map(|item_map| {
            item_map
                .applicable_types
                .into_iter()
                .map(move |in_item_id| rc::ed::EMutaItem {
                    muta_id: rc::ed::EItemId::from_i32(self.item_id),
                    in_item_id: rc::ed::EItemId::from_i32(in_item_id),
                    out_item_id: rc::ed::EItemId::from_i32(item_map.resulting_type),
                })
        }));
        extracted2.extend(self.attribute_ids.into_iter().map(|attr_range| rc::ed::EMutaAttr {
            muta_id: rc::ed::EItemId::from_i32(self.item_id),
            attr_id: rc::ed::EAttrId::from_i32(attr_range.attr_id),
            min_attr_mult: rc::ed::EFloat::from_f64(attr_range.min),
            max_attr_mult: rc::ed::EFloat::from_f64(attr_range.max),
        }));
    }
}

#[derive(Deserialize)]
struct SMutaItemMap {
    #[serde(rename = "applicableTypes")]
    applicable_types: Vec<i32>,
    #[serde(rename = "resultingType")]
    resulting_type: i32,
}

#[derive(Deserialize)]
struct SMutaAttrRange {
    #[serde(rename = "_key")]
    attr_id: i32,
    min: f64,
    max: f64,
}
