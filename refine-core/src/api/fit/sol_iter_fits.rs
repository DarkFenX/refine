use crate::{EntityIter, Fit, SolarSystem};

impl SolarSystem {
    pub fn iter_fits(&mut self) -> EntityIter<'_, Fit<'_>> {
        let fit_uids = self.u_data.fits.keys().collect();
        EntityIter::new(self, fit_uids)
    }
}
