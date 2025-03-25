use crate::sol::{FitId, svc::Svc};

impl Svc {
    pub(in crate::sol) fn add_fit(&mut self, fit_id: &FitId) {
        self.notify_fit_added(fit_id);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &FitId) {
        self.notify_fit_removed(fit_id);
    }
}
