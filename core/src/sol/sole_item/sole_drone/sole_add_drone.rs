use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::DroneInfo,
        uad::item::{Drone, Item, ItemAddMutation, MinionState},
    },
};

impl SolarSystem {
    pub fn add_drone(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> Result<DroneInfo, AddDroneError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_drone_internal(fit_key, type_id, state, mutation);
        Ok(self.get_drone_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_drone_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_item_id();
        let drone = Drone::new(&self.uad.src, item_id, type_id, fit_key, state, mutation);
        let item = Item::Drone(drone);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.drones.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
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
