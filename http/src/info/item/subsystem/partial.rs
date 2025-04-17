use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slot: Option<rc::SlotIndex>,
    pub(crate) enabled: bool,
}
impl From<&mut rc::SubsystemMut<'_>> for HSubsystemInfoPartial {
    fn from(core_subsystem: &mut rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            kind: "subsystem",
            type_id: core_subsystem.get_type_id(),
            fit_id: core_subsystem.get_fit().get_fit_id(),
            slot: core_subsystem.get_slot(),
            enabled: core_subsystem.get_state(),
        }
    }
}
