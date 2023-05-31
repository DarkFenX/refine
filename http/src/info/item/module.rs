#[derive(serde::Serialize)]
pub(crate) struct ModuleInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: reefast_core::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast_core::ReeId,
    pub type_id: reefast_core::ReeInt,
}
impl From<&reefast_core::ModuleInfo> for ModuleInfo {
    fn from(value: &reefast_core::ModuleInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.type_id,
        }
    }
}
