use crate::{Fleet, FleetId, sol::SolCore, ud::UFleetId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats of a fleet.
pub struct FleetStats<'s> {
    pub(in crate::api) sol: &'s mut SolCore,
    pub(in crate::api) uid: UFleetId,
}
impl<'s> FleetStats<'s> {
    pub fn get_fleet_id(&self) -> FleetId {
        self.sol.u_data.fleets.ext_id_by_int_id(self.uid)
    }
}
impl<'s> Fleet<'s> {
    pub fn stats(&mut self) -> FleetStats<'_> {
        self.sol.reuse.cseq_cache.clear();
        FleetStats::new(self.sol, self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FleetStats<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
}
