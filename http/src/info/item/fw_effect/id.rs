use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::FwEffectMut<'_>> for HFwEffectInfoId {
    fn from(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
        }
    }
}
