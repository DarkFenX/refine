use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(in crate::phb) item_maps: Vec<PMutaItemMap>,
}
impl FsdMerge<rc::ed::EMutaItemConv> for PMutaItemConvs {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EMutaItemConv> {
        let mut vec = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                vec.push(rc::ed::EMutaItemConv {
                    muta_id: id,
                    in_item_id: applicable_type,
                    out_item_id: item_map.result_item_id,
                })
            }
        }
        vec
    }
}
#[derive(serde::Deserialize)]
pub(in crate::phb) struct PMutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(in crate::phb) applicable_item_ids: Vec<rc::ed::EItemId>,
    #[serde(rename = "resultingType")]
    pub(in crate::phb) result_item_id: rc::ed::EItemId,
}
