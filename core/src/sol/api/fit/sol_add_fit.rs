use crate::{
    sol::{SolarSystem, api::FitMut},
    uad::{UadFit, UadFitKey},
};

impl SolarSystem {
    pub fn add_fit(&mut self) -> FitMut<'_> {
        let fit_key = self.add_fit_internal();
        FitMut::new(self, fit_key)
    }
    pub(in crate::sol::api) fn add_fit_internal(&mut self) -> UadFitKey {
        let fit_id = self.uad.fits.alloc_id();
        let uad_fit = UadFit::new(fit_id);
        let fit_key = self.uad.fits.add(uad_fit);
        self.svc.notify_fit_added(fit_key);
        fit_key
    }
}
