use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolDrone, SolItemState},
        item_info::SolProjInfo,
    },
};

pub struct SolDroneInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
    pub projs: Vec<SolProjInfo>,
}
impl SolDroneInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, state: SolItemState, projs: Vec<SolProjInfo>) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            projs,
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
            sol_drone
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
