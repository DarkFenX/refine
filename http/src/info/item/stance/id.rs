#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolStanceInfo> for HStanceInfoId {
    fn from(core_stance_info: &rc::SolStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
        }
    }
}
