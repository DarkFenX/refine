use crate::{
    err::basic::FleetFoundError,
    sol::{FleetId, SolarSystem, info::FleetInfo},
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &FleetId) -> Result<FleetInfo, GetFleetError> {
        let fleet = self.uad.fleets.get_fleet(fleet_id)?;
        Ok(FleetInfo::from(fleet))
    }
}

#[derive(Debug)]
pub enum GetFleetError {
    FleetNotFound(FleetFoundError),
}
impl std::error::Error for GetFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FleetNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FleetNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FleetFoundError> for GetFleetError {
    fn from(error: FleetFoundError) -> Self {
        Self::FleetNotFound(error)
    }
}
