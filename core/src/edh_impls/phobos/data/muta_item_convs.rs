use crate::{defs::ReeInt, edh_impls::phobos::fsd::FsdMerge, edt};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct MutaItemConvs {
    #[serde(rename = "inputOutputMapping")]
    pub(in super::super) item_maps: Vec<MutaItemMap>,
}
impl FsdMerge<edt::MutaItemConv> for MutaItemConvs {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::MutaItemConv> {
        let mut vec = Vec::new();
        for item_map in self.item_maps {
            for applicable_type in item_map.applicable_item_ids {
                vec.push(edt::MutaItemConv::new(id, applicable_type, item_map.result_item_id))
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
