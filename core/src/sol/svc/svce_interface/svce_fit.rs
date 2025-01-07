use crate::{defs::SolFitId, sol::svc::SolSvc};

impl SolSvc {
    pub(in crate::sol) fn add_fit(&mut self, fit_id: &SolFitId) {
        self.notify_fit_added(fit_id);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &SolFitId) {
        self.notify_fit_removed(fit_id);
    }
}
