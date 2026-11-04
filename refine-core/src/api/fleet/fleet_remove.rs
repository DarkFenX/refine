use itertools::Itertools;

use crate::{
    Fleet,
    sol::{CoreInner, UndoStep},
    ud::UFleetId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn remove(self) {
        self.core.internal_remove_fleet(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_fleet(&mut self, fleet_uid: UFleetId) {
        // Fits leave the fleet one by one, so that rollback can put them back via their own steps
        let fit_uids = self.u_data.fleets.get(fleet_uid).iter_fits().collect_vec();
        for fit_uid in fit_uids.into_iter() {
            self.internal_set_fit_fleet(fit_uid, None);
        }
        self.undo
            .discard(&mut self.u_data.fleets, fleet_uid, UndoStep::FleetRemoved);
    }
}
