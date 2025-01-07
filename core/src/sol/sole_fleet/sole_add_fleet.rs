use crate::sol::{info::SolFleetInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> SolFleetInfo {
        let fleet_id = self.uad.fleets.add_fleet();
        let fleet = self.uad.fleets.get_fleet(&fleet_id).unwrap();
        SolFleetInfo::from(fleet)
    }
}
