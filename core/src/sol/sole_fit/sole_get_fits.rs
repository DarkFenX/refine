use crate::sol::{SolarSystem, info::FitInfo};

impl SolarSystem {
    pub fn get_fits(&self) -> Vec<FitInfo> {
        self.uad
            .fits
            .iter_fits()
            .map(|fit| FitInfo::from_fit(&self.uad, fit))
            .collect()
    }
}
