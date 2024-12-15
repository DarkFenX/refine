use crate::sol::{fit_info::SolFitInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fit(&mut self) -> SolFitInfo {
        let fit_id = self.fits.add_fit();
        self.svcs.add_fit(&fit_id);
        let fit = self.fits.get_fit(&fit_id).unwrap();
        SolFitInfo::from(fit)
    }
}
