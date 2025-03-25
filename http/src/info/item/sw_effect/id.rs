#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SwEffectInfo> for HSwEffectInfoId {
    fn from(core_sw_effect_info: &rc::SwEffectInfo) -> Self {
        Self {
            id: core_sw_effect_info.id,
        }
    }
}
