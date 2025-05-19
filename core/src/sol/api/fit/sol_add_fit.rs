use crate::sol::{FitKey, SolarSystem, api::FitMut, uad::fit::UadFit};

impl SolarSystem {
    pub fn add_fit(&mut self) -> FitMut {
        let fit_key = self.add_fit_internal();
        FitMut::new(self, fit_key)
    }
    pub(in crate::sol::api) fn add_fit_internal(&mut self) -> FitKey {
        let fit_id = self.uad.fits.alloc_id();
        let uad_fit = UadFit::new(fit_id);
        let fit_key = self.uad.fits.add(uad_fit);
        self.svc.notify_fit_added(fit_key);
        fit_key
    }
}
