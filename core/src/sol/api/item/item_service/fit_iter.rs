use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, MutIter, Service, mut_iter::ServiceMutGenerator},
};

impl<'a> Fit<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service> {
        iter_services(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_services(&self) -> impl ExactSizeIterator<Item = Service> {
        iter_services(self.sol, self.key)
    }
    pub fn iter_services_mut(&mut self) -> MutIter<'_, ServiceMutGenerator> {
        let implant_keys = self.sol.uad.fits.get(self.key).services.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_services(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Service> {
    sol.uad
        .fits
        .get(fit_key)
        .services
        .iter()
        .map(|item_key| Service::new(sol, *item_key))
}
