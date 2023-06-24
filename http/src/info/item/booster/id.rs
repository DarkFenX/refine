#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
}
impl From<&rc::SsBoosterInfo> for HBoosterInfoId {
    fn from(core_booster_info: &rc::SsBoosterInfo) -> Self {
        Self {
            id: core_booster_info.id,
        }
    }
}
