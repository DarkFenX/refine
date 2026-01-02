use crate::{
    api::FleetMut,
    sol::SolarSystem,
    ud::{UFleet, UFleetId},
};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> FleetMut<'_> {
        let fleet_key = self.add_fleet_internal();
        FleetMut::new(self, fleet_key)
    }
    pub(in crate::api) fn add_fleet_internal(&mut self) -> UFleetId {
        let fleet_id = self.u_data.fleets.alloc_id();
        let u_fleet = UFleet::new(fleet_id);
        self.u_data.fleets.add(u_fleet)
    }
}
