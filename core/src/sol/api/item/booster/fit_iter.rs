use crate::sol::{
    FitKey, SolarSystem,
    api::{Booster, BoosterMutGenerator, Fit, FitMut, ItemMutIter},
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
    pub fn iter_boosters_mut(&mut self) -> ItemMutIter<'_, BoosterMutGenerator> {
        let booster_keys = self.sol.uad.fits.get(self.key).boosters.iter().copied().collect();
        ItemMutIter::new(self.sol, booster_keys)
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
