use itertools::Itertools;

use crate::{
    Fit, RemoveMode,
    sol::{SolCore, UndoStep},
    ud::UFitId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_fit(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_remove_fit(&mut self, fit_uid: UFitId) {
        self.internal_set_fit_fleet(fit_uid, None);
        let item_uids = self.u_data.fits.get(fit_uid).iter_direct_items().collect_vec();
        for item_uid in item_uids.into_iter() {
            self.internal_remove_item(item_uid, RemoveMode::Free).unwrap();
        }
        self.svc.notify_fit_removed(&self.u_data, fit_uid);
        self.undo.discard(&mut self.u_data.fits, fit_uid, UndoStep::FitRemoved);
    }
}
