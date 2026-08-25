use crate::{
    api::{Fleet, FleetMut},
    err::basic::FleetFoundError,
    sol::SolarSystem,
    ud::FleetId,
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<Fleet<'_>, FleetGetError> {
        let fleet_uid = self.u_data.fleets.int_id_by_ext_id_err(fleet_id)?;
        Ok(Fleet::new(self, fleet_uid))
    }
    pub fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<FleetMut<'_>, FleetGetError> {
        let fleet_uid = self.u_data.fleets.int_id_by_ext_id_err(fleet_id)?;
        Ok(FleetMut::new(self, fleet_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetGetError {
    #[error(transparent)]
    FleetNotFound(#[from] FleetFoundError),
}
