#[derive(serde::Serialize)]
pub(crate) struct ShipInfo {
    #[serde(with = "crate::util::serde_string")]
    pub item_id: reefast::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub fit_id: reefast::ReeId,
    pub type_id: reefast::ReeInt,
    pub enabled: bool,
}
impl From<&reefast::ShipInfo> for ShipInfo {
    fn from(value: &reefast::ShipInfo) -> Self {
        Self {
            item_id: value.item_id,
            fit_id: value.fit_id,
            type_id: value.type_id,
            enabled: value.enabled,
        }
    }
}
