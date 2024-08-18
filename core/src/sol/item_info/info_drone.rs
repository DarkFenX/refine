use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::{SolDrone, SolItemState},
};

pub struct SolDroneInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
}
impl SolDroneInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, state: SolItemState) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
        }
    }
}
impl From<&SolDrone> for SolDroneInfo {
    fn from(sol_drone: &SolDrone) -> Self {
        SolDroneInfo::new(
            sol_drone.get_id(),
            sol_drone.get_type_id(),
            sol_drone.get_fit_id(),
            sol_drone.get_state(),
        )
    }
}
