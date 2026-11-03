use crate::{
    Drone, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_drone(&mut self, item_id: &ItemId) -> Result<Drone<'_>, DroneGetError> {
        let sol = self.inner_mut();
        let drone_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(drone_uid).dc_drone()?;
        Ok(Drone::new(sol, drone_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DroneGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotDrone(#[from] ItemKindMatchError),
}
