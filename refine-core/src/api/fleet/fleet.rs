use crate::{
    sol::SolarSystem,
    ud::{FleetId, UFleetId},
};

pub struct Fleet<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UFleetId,
}
impl<'s> Fleet<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.uid)
    }
}

pub struct FleetMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UFleetId,
}
impl<'s> FleetMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    pub fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
    pub fn get_fleet_id(&self) -> FleetId {
        get_fleet_id(self.sol, self.uid)
    }
}

fn get_fleet_id(sol: &SolarSystem, fleet_uid: UFleetId) -> FleetId {
    sol.u_data.fleets.ext_id_by_int_id(fleet_uid)
}
