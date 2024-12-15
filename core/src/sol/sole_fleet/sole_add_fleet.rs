use crate::sol::{fleet_info::SolFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> SolFleetInfo {
        let fleet_id = self.fleets.add_fleet();
        let fleet = self.fleets.get_fleet(&fleet_id).unwrap();
        SolFleetInfo::from(fleet)
    }
}
