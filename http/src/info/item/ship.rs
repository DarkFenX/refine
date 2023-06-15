#[derive(serde::Serialize)]
pub(crate) struct HShipInfo {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) enabled: bool,
}
impl From<&rc::SsShipInfo> for HShipInfo {
    fn from(ss_ship_info: &rc::SsShipInfo) -> Self {
        Self {
            id: ss_ship_info.id,
            fit_id: ss_ship_info.fit_id,
            type_id: ss_ship_info.a_item_id,
            enabled: ss_ship_info.enabled,
        }
    }
}
