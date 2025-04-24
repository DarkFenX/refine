use crate::{
    err::basic::FleetFoundError,
    sol::{
        FleetId, FleetKey, SolarSystem,
        api::{Fleet, FleetMut},
    },
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<Fleet, GetFleetError> {
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        Ok(self.internal_get_fleet(fleet_key))
    }
    pub fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<FleetMut, GetFleetError> {
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        Ok(self.internal_get_fleet_mut(fleet_key))
    }
    pub(in crate::sol) fn internal_get_fleet(&self, fleet_key: FleetKey) -> Fleet {
        Fleet::new(self, fleet_key)
    }
    pub(in crate::sol) fn internal_get_fleet_mut(&mut self, fleet_key: FleetKey) -> FleetMut {
        FleetMut::new(self, fleet_key)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
