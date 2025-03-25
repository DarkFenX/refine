#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&rc::StanceInfo> for HStanceInfoId {
    fn from(core_stance_info: &rc::StanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
        }
    }
}
