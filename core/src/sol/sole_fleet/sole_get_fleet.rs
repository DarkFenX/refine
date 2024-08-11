use crate::{
    defs::SolFleetId,
    err::basic::FleetFoundError,
    sol::{fleet_info::SolFleetInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fleet(&self, fleet_id: &SolFleetId) -> Result<SolFleetInfo, GetFleetError> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        Ok(SolFleetInfo::from(fleet))
    }
}

#[derive(Debug)]
pub enum GetFleetError {
    FleetNotFound(FleetFoundError),
}
impl From<FleetFoundError> for GetFleetError {
    fn from(error: FleetFoundError) -> Self {
        Self::FleetNotFound(error)
    }
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
