use crate::sol::{FitKey, SolarSystem, info::FitInfo, uad::fit::UadFit};

impl SolarSystem {
    pub fn add_fit(&mut self) -> FitInfo {
        let fit_key = self.add_fit_internal();
        self.get_fit_info_internal(fit_key)
    }
    pub(in crate::sol) fn add_fit_internal(&mut self) -> FitKey {
        let fit_id = self.uad.fits.alloc_id();
        let fit = UadFit::new(fit_id);
        let fit_key = self.uad.fits.add(fit);
        self.svc.add_fit(fit_key);
        fit_key
    }
}
