#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStructureInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsStructureInfo> for HStructureInfoId {
    fn from(core_structure_info: &rc::SsStructureInfo) -> Self {
        Self {
            id: core_structure_info.id,
        }
    }
}
