use crate::sol::{SolarSystem, api::Fit};

impl SolarSystem {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit> {
        self.uad.fits.keys().map(|fit_key| Fit::new(self, fit_key))
    }
}
