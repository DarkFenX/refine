#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsStanceInfo> for HStanceInfoPartial {
    fn from(core_stance_info: &rc::SsStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
            fit_id: core_stance_info.fit_id,
            type_id: core_stance_info.a_item_id,
            enabled: core_stance_info.enabled,
        }
    }
}
