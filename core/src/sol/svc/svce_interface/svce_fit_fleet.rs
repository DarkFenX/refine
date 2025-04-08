use crate::sol::{
    FitKey,
    svc::Svc,
    uad::{Uad, fleet::Fleet},
};

impl Svc {
    pub(in crate::sol) fn add_fit_to_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_key: &FitKey) {
        self.notify_fit_added_to_fleet(uad, fleet, fit_key);
    }
    pub(in crate::sol) fn remove_fit_from_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_key: &FitKey) {
        self.notify_fit_removed_from_fleet(uad, fleet, fit_key);
    }
}
