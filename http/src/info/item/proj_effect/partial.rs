#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    pub(crate) enabled: bool,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<rc::ItemId>,
}
impl From<&rc::ProjEffectInfo> for HProjEffectInfoPartial {
    fn from(core_proj_effect_info: &rc::ProjEffectInfo) -> Self {
        Self {
            id: core_proj_effect_info.id,
            kind: "proj_effect",
            type_id: core_proj_effect_info.type_id,
            enabled: core_proj_effect_info.enabled,
            projs: core_proj_effect_info.projs.clone(),
        }
    }
}
