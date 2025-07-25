use crate::{def::FleetId, sol::SolarSystem, ud::UFleetKey};

pub struct Fleet<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UFleetKey,
}
impl<'a> Fleet<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UFleetKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

pub struct FleetMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UFleetKey,
}
impl<'a> FleetMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UFleetKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

fn get_fleet_id(sol: &SolarSystem, fleet_key: UFleetKey) -> FleetId {
    sol.u_data.fleets.id_by_key(fleet_key)
}
