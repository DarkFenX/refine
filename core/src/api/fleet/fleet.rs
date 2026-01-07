use crate::{def::FleetId, sol::SolarSystem, ud::UFleetId};

pub struct Fleet<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UFleetId,
}
impl<'a> Fleet<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UFleetId) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

pub struct FleetMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UFleetId,
}
impl<'a> FleetMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UFleetId) -> Self {
        Self { sol, key }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.key)
    }
}

fn get_fleet_id(sol: &SolarSystem, fleet_key: UFleetId) -> FleetId {
    sol.u_data.fleets.xid_by_iid(fleet_key)
}
