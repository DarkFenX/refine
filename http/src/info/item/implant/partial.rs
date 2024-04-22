#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolImplantInfo> for HImplantInfoPartial {
    fn from(core_implant_info: &rc::SolImplantInfo) -> Self {
        Self {
            id: core_implant_info.id,
            fit_id: core_implant_info.fit_id,
            type_id: core_implant_info.a_item_id,
            enabled: core_implant_info.enabled,
        }
    }
}
