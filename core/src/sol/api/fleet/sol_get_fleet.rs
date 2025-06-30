use crate::{
    def::FleetId,
    err::basic::FleetFoundError,
    sol::{
        SolarSystem,
        api::{Fleet, FleetMut},
    },
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<Fleet<'_>, GetFleetError> {
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        Ok(Fleet::new(self, fleet_key))
    }
    pub fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<FleetMut<'_>, GetFleetError> {
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        Ok(FleetMut::new(self, fleet_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
