use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: bool,
}
impl From<&mut rc::ShipMut<'_>> for HShipInfoPartial {
    fn from(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
            kind: "ship",
            type_id: core_ship.get_type_id(),
            fit_id: core_ship.get_fit().get_fit_id(),
            enabled: core_ship.get_state(),
        }
    }
}
