#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolBoosterInfo> for HBoosterInfoPartial {
    fn from(core_booster_info: &rc::SolBoosterInfo) -> Self {
        Self {
            id: core_booster_info.id,
            fit_id: core_booster_info.fit_id,
            type_id: core_booster_info.a_item_id,
            enabled: core_booster_info.enabled,
        }
    }
}
