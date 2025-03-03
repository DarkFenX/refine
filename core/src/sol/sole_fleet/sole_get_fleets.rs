use crate::sol::{SolarSystem, info::SolFleetInfo};

impl SolarSystem {
    pub fn get_fleets(&self) -> Vec<SolFleetInfo> {
        self.uad.fleets.iter_fleets().map(SolFleetInfo::from).collect()
    }
}
