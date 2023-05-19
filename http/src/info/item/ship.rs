#[derive(serde::Serialize)]
pub(crate) struct ShipInfo {
    pub item_id: String,
    pub fit_id: String,
    pub type_id: reefast::ReeInt,
    pub enabled: bool,
}
impl From<&reefast::ShipInfo> for ShipInfo {
    fn from(value: &reefast::ShipInfo) -> Self {
        Self {
            item_id: value.item_id.to_string(),
            fit_id: value.fit_id.to_string(),
            type_id: value.type_id,
            enabled: value.enabled,
        }
    }
}
