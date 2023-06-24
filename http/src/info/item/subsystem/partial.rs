#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
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
