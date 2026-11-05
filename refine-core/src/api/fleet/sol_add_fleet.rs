use crate::{
    Fleet, Sol,
    sol::{SolCore, UndoStep},
    ud::{UFleet, UFleetId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn add_fleet(&mut self) -> Fleet<'_> {
        let sol = self.inner_mut();
        let fleet_uid = sol.add_fleet_internal();
        Fleet::new(sol, fleet_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn add_fleet_internal(&mut self) -> UFleetId {
        let fleet_id = self.u_data.fleets.alloc_id();
        let u_fleet = UFleet::new(fleet_id);
        let fleet_uid = self.u_data.fleets.add(u_fleet);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FleetAdded(fleet_uid));
        }
        fleet_uid
    }
}
