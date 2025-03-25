#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slot: Option<rc::SlotIndex>,
    pub(crate) enabled: bool,
}
impl From<&rc::ImplantInfo> for HImplantInfoPartial {
    fn from(core_implant_info: &rc::ImplantInfo) -> Self {
        Self {
            id: core_implant_info.id,
            kind: "implant",
            type_id: core_implant_info.type_id,
            fit_id: core_implant_info.fit_id,
            slot: core_implant_info.slot,
            enabled: core_implant_info.enabled,
        }
    }
}
