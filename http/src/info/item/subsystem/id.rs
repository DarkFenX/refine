#[derive(serde::Serialize)]
pub(crate) struct HSubsystemInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsSubsystemInfo> for HSubsystemInfoId {
    fn from(core_subsystem_info: &rc::SsSubsystemInfo) -> Self {
        Self {
            id: core_subsystem_info.id,
        }
    }
}
