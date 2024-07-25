#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
}
impl From<&rc::SolShipInfo> for HShipInfoPartial {
    fn from(core_ship_info: &rc::SolShipInfo) -> Self {
        Self {
            id: core_ship_info.id,
            kind: "ship",
            type_id: core_ship_info.a_item_id,
            fit_id: core_ship_info.fit_id,
            enabled: core_ship_info.enabled,
        }
    }
}
