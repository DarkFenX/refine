#[derive(serde::Serialize)]
pub(crate) struct ImplantInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: reefast_core::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast_core::ReeId,
    pub type_id: reefast_core::ReeInt,
    pub enabled: bool,
}
impl From<&reefast_core::ImplantInfo> for ImplantInfo {
    fn from(value: &reefast_core::ImplantInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.type_id,
            enabled: value.enabled,
        }
    }
}
