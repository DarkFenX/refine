use crate::{
    Drone, ItemId, SolarSystem,
    err::basic::{ItemFoundError, ItemKindMatchError},
};

impl SolarSystem {
    pub fn get_drone(&mut self, item_id: &ItemId) -> Result<Drone<'_>, DroneGetError> {
        let drone_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(drone_uid).dc_drone()?;
        Ok(Drone::new(self, drone_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DroneGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
