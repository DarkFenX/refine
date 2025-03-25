use crate::sol::{SolarSystem, info::FleetInfo};

impl SolarSystem {
    pub fn get_fleets(&self) -> Vec<FleetInfo> {
        self.uad.fleets.iter_fleets().map(FleetInfo::from).collect()
    }
}
