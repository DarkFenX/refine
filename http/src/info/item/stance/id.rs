#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsStanceInfo> for HStanceInfoId {
    fn from(core_stance_info: &rc::SsStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
        }
    }
}
