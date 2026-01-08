use crate::{
    sol::SolarSystem,
    ud::{FleetId, UFleetId},
};

pub struct Fleet<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UFleetId,
}
impl<'a> Fleet<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.uid)
    }
}

pub struct FleetMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UFleetId,
}
impl<'a> FleetMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.uid)
    }
}

fn get_fleet_id(sol: &SolarSystem, fleet_uid: UFleetId) -> FleetId {
    sol.u_data.fleets.xid_by_iid(fleet_uid)
}
