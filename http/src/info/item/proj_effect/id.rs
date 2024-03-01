#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsProjEffectInfo> for HProjEffectInfoId {
    fn from(core_proj_effect_info: &rc::SsProjEffectInfo) -> Self {
        Self {
            id: core_proj_effect_info.id,
        }
    }
}
