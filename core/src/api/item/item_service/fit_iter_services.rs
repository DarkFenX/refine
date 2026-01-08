use crate::{
    api::{Fit, FitMut, MutIter, Service, ServiceMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service<'_>> {
        iter_services(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service<'_>> {
        iter_services(self.sol, self.uid)
    }
    pub fn iter_services_mut(&mut self) -> MutIter<'_, ServiceMut<'_>> {
        let service_uids = self.sol.u_data.fits.get(self.uid).services.iter().copied().collect();
        MutIter::new(self.sol, service_uids)
    }
}

fn iter_services(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Service<'_>> {
    let service_uids = sol.u_data.fits.get(fit_uid).services.iter();
    service_uids.map(|service_uid| Service::new(sol, *service_uid))
}
