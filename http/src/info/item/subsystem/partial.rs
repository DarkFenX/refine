#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slot: Option<rc::SlotIndex>,
    pub(crate) enabled: bool,
}
impl From<&rc::SubsystemInfo> for HSubsystemInfoPartial {
    fn from(core_subsystem_info: &rc::SubsystemInfo) -> Self {
        Self {
            id: core_subsystem_info.id,
            kind: "subsystem",
            type_id: core_subsystem_info.type_id,
            fit_id: core_subsystem_info.fit_id,
            slot: core_subsystem_info.slot,
            enabled: core_subsystem_info.enabled,
        }
    }
}
