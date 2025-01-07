use crate::sol::{info::SolFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fleets(&self) -> Vec<SolFleetInfo> {
        self.uad.fleets.iter_fleets().map(|v| SolFleetInfo::from(v)).collect()
    }
}
