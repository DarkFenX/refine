use crate::{
    Fit, Sol,
    sol::{SolCore, UndoStep},
    ud::{UFit, UFitId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn add_fit(&mut self) -> Fit<'_> {
        let sol = self.inner_mut();
        let fit_uid = sol.add_fit_internal();
        Fit::new(sol, fit_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn add_fit_internal(&mut self) -> UFitId {
        let fit_id = self.u_data.fits.alloc_id();
        let u_fit = UFit::new(fit_id);
        let fit_uid = self.u_data.fits.add(u_fit);
        self.svc.notify_fit_added(&self.u_data, fit_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FitAdded(fit_uid));
        }
        fit_uid
    }
    /// Puts back a fit which was removed while recording.
    pub(in crate::api) fn internal_reattach_fit(&mut self, fit_uid: UFitId) {
        self.u_data.fits.reattach(fit_uid);
        self.svc.notify_fit_added(&self.u_data, fit_uid);
    }
}
