#[derive(serde::Serialize)]
pub(crate) struct ImplantInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: reefast::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast::ReeId,
    pub type_id: reefast::ReeInt,
    pub enabled: bool,
}
impl From<&reefast::ImplantInfo> for ImplantInfo {
    fn from(value: &reefast::ImplantInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.type_id,
            enabled: value.enabled,
        }
    }
}
