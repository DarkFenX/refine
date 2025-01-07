use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        info::{SolItemMutationInfo, SolProjInfo},
        uad::item::{SolDrone, SolItemState},
    },
    src::Src,
};

pub struct SolDroneInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
    pub mutation: Option<SolItemMutationInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolDroneInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolItemState,
        mutation: Option<SolItemMutationInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            mutation,
            projs,
        }
    }
    pub(in crate::sol) fn from_drone_with_source(src: &Src, sol_drone: &SolDrone) -> Self {
        SolDroneInfo::new(
            sol_drone.get_id(),
            sol_drone.get_type_id(),
            sol_drone.get_fit_id(),
            sol_drone.get_state(),
            sol_drone.get_mutation_info(src),
            sol_drone
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
