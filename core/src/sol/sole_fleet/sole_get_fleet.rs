use crate::{
    err::basic::FleetFoundError,
    sol::{FleetId, FleetKey, SolarSystem, info::FleetInfo},
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<FleetInfo, GetFleetError> {
        let item_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        Ok(self.get_fleet_internal(item_key))
    }
    pub(in crate::sol) fn get_fleet_internal(&self, fleet_key: FleetKey) -> FleetInfo {
        let fleet = self.uad.fleets.get(fleet_key);
        FleetInfo::from_fleet(&self.uad, fleet)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
