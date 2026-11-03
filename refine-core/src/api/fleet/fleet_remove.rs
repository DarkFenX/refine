use itertools::Itertools;

use crate::{Fleet, SolarSystem, sol::UndoStep, ud::UFleetId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fleet(&mut self, fleet_uid: UFleetId) {
        // Fits leave the fleet one by one, so that rollback can put them back via their own steps
        let fit_uids = self.u_data.fleets.get(fleet_uid).iter_fits().collect_vec();
        for fit_uid in fit_uids.into_iter() {
            self.internal_set_fit_fleet(fit_uid, None);
        }
        match self.undo.is_recording() {
            true => {
                self.u_data.fleets.detach(fleet_uid);
                self.undo.record(UndoStep::FleetRemoved(fleet_uid));
            }
            false => {
                self.u_data.fleets.remove(fleet_uid);
            }
        }
    }
}

impl<'s> Fleet<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_fleet(self.uid);
    }
}
