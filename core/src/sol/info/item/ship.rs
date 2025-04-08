use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Ship};

pub struct ShipInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl ShipInfo {
    pub(in crate::sol) fn from_ship(ship: &Ship) -> Self {
        Self {
            id: ship.get_item_id(),
            type_id: ship.get_a_item_id(),
            fit_id: ship.get_fit_id(),
            enabled: ship.get_ship_state(),
        }
    }
}
