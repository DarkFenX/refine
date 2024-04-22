#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStructureInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolStructureInfo> for HStructureInfoId {
    fn from(core_structure_info: &rc::SolStructureInfo) -> Self {
        Self {
            id: core_structure_info.id,
        }
    }
}
