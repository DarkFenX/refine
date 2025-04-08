use crate::sol::{SolarSystem, info::FitInfo};

impl SolarSystem {
    pub fn add_fit(&mut self) -> FitInfo {
        let fit_id = self.uad.fits.add_fit();
        self.svc.add_fit(fit_id);
        let fit = self.uad.fits.get_fit(&fit_id).unwrap();
        FitInfo::from_fit(&self.uad, fit)
    }
}
