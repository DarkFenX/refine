use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Rig},
};

impl<'a> Fit<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig> {
        iter_rigs(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig> {
        iter_rigs(self.sol, self.key)
    }
}

fn iter_rigs(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Rig> {
    sol.uad
        .fits
        .get(fit_key)
        .rigs
        .iter()
        .map(|item_key| Rig::new(sol, *item_key))
}
