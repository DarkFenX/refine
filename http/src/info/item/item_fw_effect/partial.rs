use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    enabled: bool,
}
impl From<&mut rc::FwEffectMut<'_>> for HFwEffectInfoPartial {
    fn from(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            kind: "fw_effect",
            type_id: core_fw_effect.get_type_id(),
            fit_id: core_fw_effect.get_fit().get_fit_id(),
            enabled: core_fw_effect.get_state(),
        }
    }
}
