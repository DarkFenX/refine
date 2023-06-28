#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsImplantInfo> for HImplantInfoId {
    fn from(core_implant_info: &rc::SsImplantInfo) -> Self {
        Self {
            id: core_implant_info.id,
        }
    }
}
