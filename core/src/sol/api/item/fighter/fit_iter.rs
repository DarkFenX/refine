use crate::sol::{
    FitKey, SolarSystem,
    api::{Fighter, Fit, FitMut},
};

impl<'a> Fit<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter> {
        iter_fighters(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter> {
        iter_fighters(self.sol, self.key)
    }
}

fn iter_fighters(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Fighter> {
    sol.uad
        .fits
        .get(fit_key)
        .fighters
        .iter()
        .map(|item_key| Fighter::new(sol, *item_key))
}
