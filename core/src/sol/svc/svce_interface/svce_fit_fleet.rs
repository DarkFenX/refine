use crate::{
    defs::SolFitId,
    sol::{fleet::SolFleet, svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(in crate::sol) fn add_fit_to_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_added_to_fleet(sol_view, fleet, fit_id);
    }
    pub(in crate::sol) fn remove_fit_from_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_removed_from_fleet(sol_view, fleet, fit_id);
    }
}
