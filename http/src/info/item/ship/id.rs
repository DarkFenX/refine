#[derive(serde::Serialize)]
pub(crate) struct HShipInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsShipInfo> for HShipInfoId {
    fn from(core_ship_info: &rc::SsShipInfo) -> Self {
        Self { id: core_ship_info.id }
    }
}
