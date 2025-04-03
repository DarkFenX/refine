use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Ship};

pub struct ShipInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl From<&Ship> for ShipInfo {
    fn from(sol_ship: &Ship) -> Self {
        Self {
            id: sol_ship.get_item_id(),
            type_id: sol_ship.get_a_item_id(),
            fit_id: sol_ship.get_fit_id(),
            enabled: sol_ship.get_ship_state(),
        }
    }
}
