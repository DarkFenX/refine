#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsImplantInfo> for HImplantInfoPartial {
    fn from(core_implant_info: &rc::SsImplantInfo) -> Self {
        Self {
            id: core_implant_info.id,
            fit_id: core_implant_info.fit_id,
            type_id: core_implant_info.a_item_id,
            enabled: core_implant_info.enabled,
        }
    }
}
