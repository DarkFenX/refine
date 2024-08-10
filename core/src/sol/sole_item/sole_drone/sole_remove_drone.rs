use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn remove_drone(&mut self, item_id: &SolItemId) -> Result<(), RemoveDroneError> {
        // Just check if everything is correct
        let item = self.items.get_item(item_id)?;
        item.get_drone()?;
        // Remove incoming projections
        self.remove_incoming_projections(item_id);
        // Remove drone from services
        let item = self.items.get_item(item_id).unwrap();
        let drone = item.get_drone().unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Remove drone from skeleton
        let fit = self.fits.get_fit_mut(&drone.get_fit_id()).unwrap();
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
