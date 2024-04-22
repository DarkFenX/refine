#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStructureInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolStructureInfo> for HStructureInfoPartial {
    fn from(core_structure_info: &rc::SolStructureInfo) -> Self {
        Self {
            id: core_structure_info.id,
            fit_id: core_structure_info.fit_id,
            type_id: core_structure_info.a_item_id,
            enabled: core_structure_info.enabled,
        }
    }
}
