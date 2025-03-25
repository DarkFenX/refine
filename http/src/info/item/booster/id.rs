#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::BoosterInfo> for HBoosterInfoId {
    fn from(core_booster_info: &rc::BoosterInfo) -> Self {
        Self {
            id: core_booster_info.id,
        }
    }
}
