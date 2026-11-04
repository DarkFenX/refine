use crate::{
    Fit, FitSecStatus,
    sol::{CoreInner, UndoStep},
    ud::UFitId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn set_sec_status(&mut self, sec_status: FitSecStatus) {
        self.core.internal_set_fit_sec_status(self.uid, sec_status);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_fit_sec_status(&mut self, fit_uid: UFitId, sec_status: FitSecStatus) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let old_sec_status = u_fit.sec_status;
        if old_sec_status == sec_status {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FitSecStatus(fit_uid, old_sec_status));
        }
        u_fit.sec_status = sec_status;
        if let Some(ship_uid) = u_fit.ship {
            self.svc.notify_ship_sec_status_changed(&self.u_data, ship_uid);
        }
    }
}
