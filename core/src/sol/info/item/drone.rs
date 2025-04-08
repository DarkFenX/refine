use crate::sol::{
    FitId, ItemId, ItemTypeId,
    info::{ItemMutationInfo, ProjInfo},
    uad::{
        Uad,
        item::{Drone, MinionState},
    },
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
    pub(in crate::sol) fn from_drone(uad: &Uad, drone: &Drone) -> Self {
        Self {
            id: drone.get_item_id(),
            type_id: drone.get_a_item_id(),
            fit_id: uad.fits.id_by_key(drone.get_fit_key()),
            state: drone.get_drone_state(),
            mutation: drone.get_mutation_info(&uad.src),
            projs: drone
                .get_projs()
                .iter()
                .map(|(&projectee_item_key, &range)| ProjInfo {
                    item_id: uad.items.id_by_key(projectee_item_key),
                    range,
                })
                .collect(),
        }
    }
}
