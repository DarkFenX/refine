use crate::{
    api::{Fighter, FighterMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::{UFitKey, UItemKey},
};

impl<'a> Fit<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter<'_>> {
        iter_fighters(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter<'_>> {
        iter_fighters(self.sol, self.key)
    }
    pub fn iter_fighters_mut(&mut self) -> MutIter<'_, FighterMut<'_>> {
        let fighter_keys = self.sol.u_data.fits.get(self.key).fighters.iter().copied().collect();
        MutIter::new(self.sol, fighter_keys)
    }
}

fn iter_fighters(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Fighter<'_>> {
    let fighter_keys = sol.u_data.fits.get(fit_key).fighters.iter();
    fighter_keys.map(|fighter_key| Fighter::new(sol, *fighter_key))
}
