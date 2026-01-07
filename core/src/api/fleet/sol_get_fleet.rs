use crate::{
    api::{Fleet, FleetMut},
    def::FleetId,
    err::basic::FleetFoundError,
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<Fleet<'_>, GetFleetError> {
        let fleet_key = self.u_data.fleets.iid_by_xid_err(fleet_id)?;
        Ok(Fleet::new(self, fleet_key))
    }
    pub fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<FleetMut<'_>, GetFleetError> {
        let fleet_key = self.u_data.fleets.iid_by_xid_err(fleet_id)?;
        Ok(FleetMut::new(self, fleet_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
