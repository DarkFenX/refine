use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::DroneInfo,
        uad::item::{Drone, Item, ItemAddMutation, MinionState},
    },
};

impl SolarSystem {
    pub fn add_drone(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> Result<DroneInfo, AddDroneError> {
        let item_id = self.uad.items.alloc_item_id();
        let drone = Drone::new(&self.uad.src, item_id, type_id, fit_id, state, mutation);
        let info = self.make_drone_info(&drone);
        let item = Item::Drone(drone);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.drones.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddDroneError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddDroneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddDroneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddDroneError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
