use itertools::Itertools;

use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_drone(&mut self, item_id: &SolItemId) -> Result<(), RemoveDroneError> {
        // Just check if everything is correct
        let item = self.items.get_item(item_id)?;
        let drone = item.get_drone()?;
        let fit_id = drone.get_fit_id();
        // Remove outgoing projections
        let proj_outgoing = drone.get_projs().iter_items().map(|v| *v).collect_vec();
        for projectee_item_id in proj_outgoing {
            self.remove_drone_proj(item_id, &projectee_item_id).unwrap();
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove drone from services
        self.remove_item_id_from_svcs(item_id);
        // Remove drone from skeleton
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        fit.drones.remove(item_id);
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveDroneError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
}
impl std::error::Error for RemoveDroneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveDroneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveDroneError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveDroneError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
