#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::ItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsBoosterInfo> for HBoosterInfoPartial {
    fn from(core_booster_info: &rc::SsBoosterInfo) -> Self {
        Self {
            id: core_booster_info.id,
            fit_id: core_booster_info.fit_id,
            type_id: core_booster_info.a_item_id,
            enabled: core_booster_info.enabled,
        }
    }
}
