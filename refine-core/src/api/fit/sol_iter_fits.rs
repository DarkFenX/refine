use crate::{
    api::{Fit, FitMut, MutIter},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        self.u_data.fits.keys().map(|fit_uid| Fit::new(self, fit_uid))
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMut<'_>> {
        let fit_uids = self.u_data.fits.keys().collect();
        MutIter::new(self, fit_uids)
    }
}
