use crate::{
    api::{Booster, BoosterMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster<'_>> {
        iter_boosters(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster<'_>> {
        iter_boosters(self.sol, self.key)
    }
    pub fn iter_boosters_mut(&mut self) -> MutIter<'_, BoosterMut<'_>> {
        let booster_keys = self.sol.u_data.fits.get(self.key).boosters.iter().copied().collect();
        MutIter::new(self.sol, booster_keys)
    }
}

fn iter_boosters(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Booster<'_>> {
    let booster_keys = sol.u_data.fits.get(fit_key).boosters.iter();
    booster_keys.map(|booster_key| Booster::new(sol, *booster_key))
}
