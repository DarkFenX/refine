use crate::sol::{fit_info::SolFitInfo, SolarSystem};

impl SolarSystem {
    pub fn get_fits(&self) -> Vec<SolFitInfo> {
        self.fits.iter_fits().map(|v| v.into()).collect()
    }
}
