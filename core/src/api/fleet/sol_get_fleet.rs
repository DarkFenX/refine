use crate::{
    api::{Fleet, FleetMut},
    err::basic::FleetFoundError,
    sol::SolarSystem,
    ud::FleetId,
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<Fleet<'_>, GetFleetError> {
        let fleet_uid = self.u_data.fleets.iid_by_xid_err(fleet_id)?;
        Ok(Fleet::new(self, fleet_uid))
    }
    pub fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<FleetMut<'_>, GetFleetError> {
        let fleet_uid = self.u_data.fleets.iid_by_xid_err(fleet_id)?;
        Ok(FleetMut::new(self, fleet_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
