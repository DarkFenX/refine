#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStructureInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsStructureInfo> for HStructureInfoPartial {
    fn from(core_structure_info: &rc::SsStructureInfo) -> Self {
        Self {
            id: core_structure_info.id,
            fit_id: core_structure_info.fit_id,
            type_id: core_structure_info.a_item_id,
            enabled: core_structure_info.enabled,
        }
    }
}
