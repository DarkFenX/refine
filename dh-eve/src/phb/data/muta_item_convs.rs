use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PMutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(in crate::phb) item_maps: Vec<PMutaItemMap>,
}
impl FsdMerge<rc::ed::EMutaItemConv> for PMutaItemConvs {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EMutaItemConv> {
        let mut vec = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                vec.push(rc::ed::EMutaItemConv::new(id, applicable_type, item_map.result_item_id))
            }
        }
        vec
    }
}
#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PMutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(in crate::phb) applicable_item_ids: Vec<rc::ReeInt>,
    #[serde(rename = "resultingType")]
    pub(in crate::phb) result_item_id: rc::ReeInt,
}
