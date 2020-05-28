use crate::{defines::ReeInt, dh};

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct MutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(in super::super) item_maps: Vec<MutaItemMap>,
}
impl FsdMerge<dh::MutaItemConv> for MutaItemConvs {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::MutaItemConv> {
        let mut vec = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                vec.push(dh::MutaItemConv::new(id, applicable_type, item_map.result_item_id))
            }
        }
        vec
    }
}
#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct MutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(in super::super) applicable_item_ids: Vec<ReeInt>,
    #[serde(rename = "resultingType")]
    pub(in super::super) result_item_id: ReeInt,
}
