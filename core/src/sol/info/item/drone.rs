use crate::{
    sol::{
        FitId, ItemId, ItemTypeId,
        info::{ItemMutationInfo, ProjInfo},
        uad::item::{Drone, MinionState},
    },
    src::Src,
};

pub struct DroneInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: MinionState,
    pub mutation: Option<ItemMutationInfo>,
    pub projs: Vec<ProjInfo>,
}
impl DroneInfo {
    pub(in crate::sol) fn from_drone_with_source(src: &Src, sol_drone: &Drone) -> Self {
        Self {
            id: sol_drone.get_item_id(),
            type_id: sol_drone.get_a_item_id(),
            fit_id: sol_drone.get_fit_id(),
            state: sol_drone.get_drone_state(),
            mutation: sol_drone.get_mutation_info(src),
            projs: sol_drone
                .get_projs()
                .iter()
                .map(|(&item_id, &range)| ProjInfo { item_id, range })
                .collect(),
        }
    }
}
