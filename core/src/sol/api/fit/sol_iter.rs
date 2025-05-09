use crate::sol::{
    SolarSystem,
    api::{Fit, FitMut, MutIter},
};

impl SolarSystem {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit> {
        self.uad.fits.keys().map(|fit_key| Fit::new(self, fit_key))
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMut<'_>> {
        let fit_keys = self.uad.fits.keys().collect();
        MutIter::new(self, fit_keys)
    }
}
