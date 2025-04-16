use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Service},
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
}

fn iter_services(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Service> {
    sol.uad
        .fits
        .get(fit_key)
        .services
        .iter()
        .map(|item_key| Service::new(sol, *item_key))
}
