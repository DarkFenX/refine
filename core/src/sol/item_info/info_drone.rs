use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::{SolDrone, SolItemState},
};

pub struct SolDroneInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub state: SolItemState,
}
impl SolDroneInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, state: SolItemState) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
        }
    }
}
impl From<&SolDrone> for SolDroneInfo {
    fn from(sol_drone: &SolDrone) -> Self {
        SolDroneInfo::new(
            sol_drone.get_id(),
            sol_drone.get_fit_id(),
            sol_drone.get_type_id(),
            sol_drone.get_state(),
        )
    }
}
