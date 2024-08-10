use crate::sol::{fleet_info::SolFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fleets(&self) -> Vec<SolFleetInfo> {
        self.fleets.iter_fleets().map(|v| SolFleetInfo::from(v)).collect()
    }
}
