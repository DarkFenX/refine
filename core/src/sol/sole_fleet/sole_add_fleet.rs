use crate::{
    err::basic::FleetAllocError,
    sol::{fleet_info::SolFleetInfo, SolarSystem},
};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> Result<SolFleetInfo, AddFleetError> {
        let fleet_id = self.fleets.add_fleet()?;
        let fleet = self.fleets.get_fleet(&fleet_id).unwrap();
        Ok(SolFleetInfo::from(fleet))
    }
}

#[derive(Debug)]
pub enum AddFleetError {
    FleetIdAllocFailed(FleetAllocError),
}
impl std::error::Error for AddFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FleetIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FleetIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<FleetAllocError> for AddFleetError {
    fn from(error: FleetAllocError) -> Self {
        Self::FleetIdAllocFailed(error)
    }
}
