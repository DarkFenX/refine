#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsSwEffectInfo> for HSwEffectInfoId {
    fn from(core_sw_effect_info: &rc::SsSwEffectInfo) -> Self {
        Self {
            id: core_sw_effect_info.id,
        }
    }
}
