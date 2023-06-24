#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
}
impl From<&rc::SsSwEffectInfo> for HSwEffectInfoId {
    fn from(core_sw_effect_info: &rc::SsSwEffectInfo) -> Self {
        Self {
            id: core_sw_effect_info.id,
        }
    }
}
