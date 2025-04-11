use crate::sol::{SolarSystem, info::FleetInfo};

impl SolarSystem {
    pub fn get_fleet_infos(&self) -> Vec<FleetInfo> {
        self.uad
            .fleets
            .values()
            .map(|fleet| FleetInfo::from_fleet(&self.uad, fleet))
            .collect()
    }
}
