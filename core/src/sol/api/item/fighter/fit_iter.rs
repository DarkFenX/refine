use crate::sol::{
    FitKey, SolarSystem,
    api::{Fighter, FighterMutGenerator, Fit, FitMut, ItemMutIter},
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
    pub fn iter_fighters_mut(&mut self) -> ItemMutIter<'_, FighterMutGenerator> {
        let implant_keys = self.sol.uad.fits.get(self.key).fighters.iter().copied().collect();
        ItemMutIter::new(self.sol, implant_keys)
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
