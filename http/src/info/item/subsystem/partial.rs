#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
}
impl From<&rc::SsSubsystemInfo> for HSubsystemInfoPartial {
    fn from(core_subsystem_info: &rc::SsSubsystemInfo) -> Self {
        Self {
            id: core_subsystem_info.id,
            fit_id: core_subsystem_info.fit_id,
            type_id: core_subsystem_info.a_item_id,
            enabled: core_subsystem_info.enabled,
        }
    }
}
