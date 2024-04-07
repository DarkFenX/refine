#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    pub(crate) tgts: Vec<rc::SsItemId>,
}
impl From<&rc::SsProjEffectInfo> for HProjEffectInfoPartial {
    fn from(core_proj_effect_info: &rc::SsProjEffectInfo) -> Self {
        Self {
            id: core_proj_effect_info.id,
            type_id: core_proj_effect_info.a_item_id,
            enabled: core_proj_effect_info.enabled,
            tgts: core_proj_effect_info.tgts.clone(),
        }
    }
}
