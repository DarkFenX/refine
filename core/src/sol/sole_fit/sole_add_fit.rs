use crate::sol::{SolarSystem, info::SolFitInfo};

impl SolarSystem {
    pub fn add_fit(&mut self) -> SolFitInfo {
        let fit_id = self.uad.fits.add_fit();
        self.svc.add_fit(&fit_id);
        let fit = self.uad.fits.get_fit(&fit_id).unwrap();
        SolFitInfo::from(fit)
    }
}
