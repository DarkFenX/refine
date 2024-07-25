#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolSubsystemInfo> for HSubsystemInfoPartial {
    fn from(core_subsystem_info: &rc::SolSubsystemInfo) -> Self {
        Self {
            id: core_subsystem_info.id,
            kind: "subsystem",
            type_id: core_subsystem_info.a_item_id,
            fit_id: core_subsystem_info.fit_id,
            enabled: core_subsystem_info.enabled,
        }
    }
}
