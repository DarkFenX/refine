use crate::sol::{SolarSystem, info::SolFitInfo};

impl SolarSystem {
    pub fn get_fits(&self) -> Vec<SolFitInfo> {
        self.uad.fits.iter_fits().map(|v| v.into()).collect()
    }
}
