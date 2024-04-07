#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsBoosterInfo> for HBoosterInfoId {
    fn from(core_booster_info: &rc::SsBoosterInfo) -> Self {
        Self {
            id: core_booster_info.id,
        }
    }
}
