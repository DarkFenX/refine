use crate::sol::{
    SolarSystem,
    api::{Fit, MutIter, mut_iter::FitMutGenerator},
};

impl SolarSystem {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit> {
        self.uad.fits.keys().map(|fit_key| Fit::new(self, fit_key))
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMutGenerator> {
        let fit_keys = self.uad.fits.keys().collect();
        MutIter::new(self, fit_keys)
    }
}
