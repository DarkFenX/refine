#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolProjEffectInfo> for HProjEffectInfoId {
    fn from(core_proj_effect_info: &rc::SolProjEffectInfo) -> Self {
        Self {
            id: core_proj_effect_info.id,
        }
    }
}
