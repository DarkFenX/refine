use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Implant},
};

impl<'a> Fit<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant> {
        iter_implants(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant> {
        iter_implants(self.sol, self.key)
    }
}

fn iter_implants(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Implant> {
    sol.uad
        .fits
        .get(fit_key)
        .implants
        .iter()
        .map(|item_key| Implant::new(sol, *item_key))
}
