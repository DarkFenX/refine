use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::SwEffectMut<'_>> for HSwEffectInfoId {
    fn from(core_sw_effect: &mut rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
        }
    }
}
