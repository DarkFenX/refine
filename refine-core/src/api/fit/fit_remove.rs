use itertools::Itertools;

use crate::{
    api::{Fit, RemoveMode},
    sol::{SolarSystem, UndoStep},
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fit(&mut self, fit_uid: UFitId) {
        self.internal_set_fit_fleet(fit_uid, None);
        let item_uids = self.u_data.fits.get(fit_uid).iter_direct_items().collect_vec();
        for item_uid in item_uids.into_iter() {
            self.internal_remove_item(item_uid, RemoveMode::Free).unwrap();
        }
        self.svc.notify_fit_removed(fit_uid);
        match self.undo.is_recording() {
            true => {
                self.u_data.fits.detach(fit_uid);
                self.undo.record(UndoStep::FitRemoved(fit_uid));
            }
            false => {
                self.u_data.fits.remove(fit_uid);
            }
        }
    }
}

impl<'s> Fit<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_fit(self.uid);
    }
}
