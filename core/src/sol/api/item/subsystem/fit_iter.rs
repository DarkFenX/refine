use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Subsystem},
};

impl<'a> Fit<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem> {
        iter_subsystems(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem> {
        iter_subsystems(self.sol, self.key)
    }
}

fn iter_subsystems(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Subsystem> {
    sol.uad
        .fits
        .get(fit_key)
        .subsystems
        .iter()
        .map(|item_key| Subsystem::new(sol, *item_key))
}
