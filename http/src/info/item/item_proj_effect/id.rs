use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::ProjEffectMut<'_>> for HProjEffectInfoId {
    fn from(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
        }
    }
}
