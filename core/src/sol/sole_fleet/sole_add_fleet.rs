use crate::sol::{FleetKey, SolarSystem, info::FleetInfo, uad::fleet::UadFleet};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> FleetInfo {
        let fleet_key = self.add_fleet_internal();
        self.get_fleet_info_internal(fleet_key)
    }
    pub(in crate::sol) fn add_fleet_internal(&mut self) -> FleetKey {
        let fleet_id = self.uad.fleets.alloc_id();
        let fleet = UadFleet::new(fleet_id);
        self.uad.fleets.add(fleet)
    }
}
