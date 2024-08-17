#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HStanceInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolStanceInfo> for HStanceInfoPartial {
    fn from(core_stance_info: &rc::SolStanceInfo) -> Self {
        Self {
            id: core_stance_info.id,
            kind: "stance",
            type_id: core_stance_info.type_id,
            fit_id: core_stance_info.fit_id,
            enabled: core_stance_info.enabled,
        }
    }
}
