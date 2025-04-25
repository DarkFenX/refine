use rc::ItemCommon;

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
impl From<&mut rc::ProjEffectMut<'_>> for HProjEffectInfoPartial {
    fn from(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            kind: "proj_effect",
            type_id: core_proj_effect.get_type_id(),
            enabled: core_proj_effect.get_state(),
            projs: core_proj_effect
                .iter_projs()
                .map(|proj| proj.get_projectee_item_id())
                .collect(),
        }
    }
}
