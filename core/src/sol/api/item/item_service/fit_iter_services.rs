use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, MutIter, Service, ServiceMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service<'_>> {
        iter_services(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service<'_>> {
        iter_services(self.sol, self.key)
    }
    pub fn iter_services_mut(&mut self) -> MutIter<'_, ServiceMut<'_>> {
        let implant_keys = self.sol.u_data.fits.get(self.key).services.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_services(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Service<'_>> {
    sol.u_data
        .fits
        .get(fit_key)
        .services
        .iter()
        .map(|item_key| Service::new(sol, *item_key))
}
