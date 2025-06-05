use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HShipInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::ShipMut<'_>> for HShipInfoId {
    fn from(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
        }
    }
}
