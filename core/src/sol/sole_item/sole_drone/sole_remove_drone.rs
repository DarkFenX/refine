use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone(&mut self, item_id: &ItemId) -> Result<(), RemoveDroneError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_drone_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_drone_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        // Just check if everything is correct
        let item = self.uad.items.get(item_key);
        let drone = item.get_drone()?;
        let fit_key = drone.get_fit_key();
        // Remove outgoing projections
        for &projectee_item_key in drone.get_projs().iter_projectee_item_keys() {
            // Update services
            let projectee_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
            // Update user data - do not update info on drone, because drone will be discarded
            // anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove incoming projections
        self.remove_incoming_projections(item_key);
        // Remove drone from services
        self.remove_item_key_from_svc(item_key);
        // Remove drone from user data
        let fit = self.uad.fits.get_mut(fit_key);
        fit.drones.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveDroneError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
