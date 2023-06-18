#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
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
