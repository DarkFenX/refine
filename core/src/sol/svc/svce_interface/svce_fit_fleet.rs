use crate::{
    defs::SolFitId,
    sol::{
        svc::SolSvc,
        uad::{SolUad, fleet::SolFleet},
    },
};

impl SolSvc {
    pub(in crate::sol) fn add_fit_to_fleet(&mut self, uad: &SolUad, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_added_to_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol) fn remove_fit_from_fleet(&mut self, uad: &SolUad, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_removed_from_fleet(uad, fleet, fit_id);
    }
}
