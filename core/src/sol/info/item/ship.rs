use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::SolShip,
};

pub struct SolShipInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolShipInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolShip> for SolShipInfo {
    fn from(sol_ship: &SolShip) -> Self {
        SolShipInfo::new(
            sol_ship.get_id(),
            sol_ship.get_type_id(),
            sol_ship.get_fit_id(),
            sol_ship.get_ship_state(),
        )
    }
}
