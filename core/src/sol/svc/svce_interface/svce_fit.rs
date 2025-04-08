use crate::sol::{FitKey, svc::Svc};

impl Svc {
    pub(in crate::sol) fn add_fit(&mut self, fit_key: FitKey) {
        self.notify_fit_added(fit_key);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_key: FitKey) {
        self.notify_fit_removed(fit_key);
    }
}
