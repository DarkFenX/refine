use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone(&mut self, item_id: &ItemId) -> Result<(), RemoveDroneError> {
        // Just check if everything is correct
        let item = self.uad.items.get_item(item_id)?;
        let drone = item.get_drone()?;
        let fit_id = drone.get_fit_id();
        // Remove outgoing projections
        for projectee_item_id in drone.get_projs().iter_items() {
            // Update services
            let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
            self.svc.remove_item_projection(&self.uad, item, projectee_item);
            // Update user data - do not update info on drone, because drone will be discarded
            // anyway
            self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove drone from services
        self.remove_item_id_from_svc(item_id);
        // Remove drone from user data
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.drones.remove(item_id);
        self.uad.items.remove_item(item_id);
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
