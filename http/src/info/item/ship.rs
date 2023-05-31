#[derive(serde::Serialize)]
pub(crate) struct ShipInfo {
    #[serde(with = "crate::util::serde_string")]
    pub id: reefast_core::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast_core::ReeId,
    pub type_id: reefast_core::ReeInt,
    pub enabled: bool,
}
impl From<&reefast_core::ShipInfo> for ShipInfo {
    fn from(value: &reefast_core::ShipInfo) -> Self {
        Self {
            id: value.id,
            fit_id: value.fit_id,
            type_id: value.type_id,
            enabled: value.enabled,
        }
    }
}
