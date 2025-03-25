use crate::sol::{SolarSystem, info::FleetInfo};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> FleetInfo {
        let fleet_id = self.uad.fleets.add_fleet();
        let fleet = self.uad.fleets.get_fleet(&fleet_id).unwrap();
        FleetInfo::from(fleet)
    }
}
