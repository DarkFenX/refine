use crate::{
    api::Fleet,
    sol::{SolarSystem, UndoStep},
    ud::{UFleet, UFleetId},
};

impl SolarSystem {
    pub fn add_fleet(&mut self) -> Fleet<'_> {
        let fleet_uid = self.add_fleet_internal();
        Fleet::new(self, fleet_uid)
    }
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
