use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::{SolDrone, SolItemState},
};

pub struct SolDroneInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub state: SolItemState,
}
impl SolDroneInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, state: SolItemState) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
        }
    }
}
impl From<&SolDrone> for SolDroneInfo {
    fn from(sol_drone: &SolDrone) -> Self {
        SolDroneInfo::new(sol_drone.id, sol_drone.fit_id, sol_drone.a_item_id, sol_drone.state)
    }
}
