use crate::{EntityIter, Fit, Service};

impl<'s> Fit<'s> {
    pub fn iter_services(&mut self) -> EntityIter<'_, Service<'_>> {
        let service_uids = self.sol.u_data.fits.get(self.uid).services.iter().copied().collect();
        EntityIter::new(self.sol, service_uids)
    }
}
