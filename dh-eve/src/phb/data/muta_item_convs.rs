use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(crate) item_maps: Vec<MutaItemMap>,
}
impl FsdMerge<rc::ed::EMutaItemConv> for MutaItemConvs {
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
pub(crate) struct MutaItemMap {
    #[serde(rename = "applicableTypes")]
    pub(crate) applicable_item_ids: Vec<rc::ReeInt>,
    #[serde(rename = "resultingType")]
    pub(crate) result_item_id: rc::ReeInt,
}
