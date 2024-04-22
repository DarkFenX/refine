#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolShipInfo> for HShipInfoId {
    fn from(core_ship_info: &rc::SolShipInfo) -> Self {
        Self { id: core_ship_info.id }
    }
}
