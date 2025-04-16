use crate::sol::{
    FitKey, SolarSystem,
    api::{Booster, Fit, FitMut},
};

impl<'a> Fit<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster> {
        iter_boosters(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster> {
        iter_boosters(self.sol, self.key)
    }
}

fn iter_boosters(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Booster> {
    sol.uad
        .fits
        .get(fit_key)
        .boosters
        .iter()
        .map(|item_key| Booster::new(sol, *item_key))
}
