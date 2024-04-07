#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsStanceInfo> for HStanceInfoId {
    fn from(core_stance_info: &rc::SsStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
        }
    }
}
