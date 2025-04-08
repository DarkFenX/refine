use crate::sol::{FleetKey, SolarSystem, info::FleetInfo, uad::fleet::Fleet};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> FleetInfo {
        let fleet_key = self.add_fleet_internal();
        self.get_fleet_internal(fleet_key)
    }
    pub(in crate::sol) fn add_fleet_internal(&mut self) -> FleetKey {
        let fleet_id = self.uad.fleets.alloc_fleet_id();
        let fleet = Fleet::new(fleet_id);
        let fleet_key = self.uad.fleets.add(fleet);
        fleet_key
    }
}
