#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsShipInfo> for HShipInfoId {
    fn from(core_ship_info: &rc::SsShipInfo) -> Self {
        Self { id: core_ship_info.id }
    }
}
