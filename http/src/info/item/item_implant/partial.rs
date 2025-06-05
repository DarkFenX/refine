use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<rc::SlotIndex>,
    enabled: bool,
}
impl From<&mut rc::ImplantMut<'_>> for HImplantInfoPartial {
    fn from(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
            kind: "implant",
            type_id: core_implant.get_type_id(),
            fit_id: core_implant.get_fit().get_fit_id(),
            slot: core_implant.get_slot(),
            enabled: core_implant.get_state(),
        }
    }
}
