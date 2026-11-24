use crate::{Fleet, FleetId, sol::SolCore, ud::UFleetId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats of a fleet.
pub struct FleetStats<'sol> {
    pub(in crate::api) sol: &'sol mut SolCore,
    pub(in crate::api) uid: UFleetId,
}
impl<'sol> FleetStats<'sol> {
    pub fn get_fleet_id(&self) -> FleetId {
        self.sol.u_data.fleets.ext_id_by_int_id(self.uid)
    }
}
impl<'sol> Fleet<'sol> {
    pub fn stats(&mut self) -> FleetStats<'_> {
        self.sol.reuse.cseq_cache.clear();
        FleetStats::new(self.sol, self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> FleetStats<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UFleetId) -> Self {
        Self { sol, uid }
    }
}
