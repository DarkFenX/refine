use crate::{
    sol::SolarSystem,
    ud::{FleetId, UFleetId},
};

pub struct Fleet<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UFleetId,
}
impl<'s> Fleet<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
    pub fn get_fleet_id(&self) -> FleetId {
        self.sol.u_data.fleets.ext_id_by_int_id(self.uid)
    }
}
