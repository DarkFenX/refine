use crate::sol::{FleetId, FleetKey, SolarSystem};

pub struct Fleet<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: FleetKey,
}
impl<'a> Fleet<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: FleetKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

pub struct FleetMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: FleetKey,
}
impl<'a> FleetMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: FleetKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

fn get_fleet_id(sol: &SolarSystem, fleet_key: FleetKey) -> FleetId {
    sol.uad.fleets.id_by_key(fleet_key)
}
