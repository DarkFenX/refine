#[derive(serde::Serialize)]
pub(crate) struct HImplantInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) enabled: bool,
}
impl From<&rc::SsImplantInfo> for HImplantInfo {
    fn from(ss_implant_info: &rc::SsImplantInfo) -> Self {
        Self {
            id: ss_implant_info.id,
            fit_id: ss_implant_info.fit_id,
            type_id: ss_implant_info.a_item_id,
            enabled: ss_implant_info.enabled,
        }
    }
}
