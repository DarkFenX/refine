use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::UadShip},
};

pub struct ShipInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl ShipInfo {
    pub(in crate::sol) fn from_ship(uad: &Uad, ship: &UadShip) -> Self {
        Self {
            id: ship.get_item_id(),
            type_id: ship.get_a_item_id(),
            fit_id: uad.fits.id_by_key(ship.get_fit_key()),
            enabled: ship.get_ship_state(),
        }
    }
}
