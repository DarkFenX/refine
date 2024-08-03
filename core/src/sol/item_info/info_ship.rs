use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolShip,
};

pub struct SolShipInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolShipInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolShip> for SolShipInfo {
    fn from(sol_ship: &SolShip) -> Self {
        SolShipInfo::new(
            sol_ship.get_id(),
            sol_ship.get_fit_id(),
            sol_ship.get_a_item_id(),
            sol_ship.get_bool_state(),
        )
    }
}
