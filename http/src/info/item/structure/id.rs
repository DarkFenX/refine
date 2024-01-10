#[derive(serde::Serialize)]
pub(crate) struct HStructureInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsStructureInfo> for HStructureInfoId {
    fn from(core_structure_info: &rc::SsStructureInfo) -> Self {
        Self {
            id: core_structure_info.id,
        }
    }
}
