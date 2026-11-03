use crate::{
    api::Fit,
    sol::{SolarSystem, UndoStep},
    ud::{UFit, UFitId},
};

impl SolarSystem {
    pub fn add_fit(&mut self) -> Fit<'_> {
        let fit_uid = self.add_fit_internal();
        Fit::new(self, fit_uid)
    }
    pub(in crate::api) fn add_fit_internal(&mut self) -> UFitId {
        let fit_id = self.u_data.fits.alloc_id();
        let u_fit = UFit::new(fit_id);
        let fit_uid = self.u_data.fits.add(u_fit);
        self.svc.notify_fit_added(fit_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FitAdded(fit_uid));
        }
        fit_uid
    }
    /// Puts back a fit which was removed while recording.
    pub(in crate::api) fn internal_reattach_fit(&mut self, fit_uid: UFitId) {
        self.u_data.fits.reattach(fit_uid);
        self.svc.notify_fit_added(fit_uid);
    }
}
