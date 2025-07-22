use crate::{
    sol::{
        SolarSystem,
        api::{Fighter, FighterMut, Fit, FitMut, MutIter},
    },
    uad::UadFitKey,
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
        let implant_keys = self.sol.uad.fits.get(self.key).fighters.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_fighters(sol: &SolarSystem, fit_key: UadFitKey) -> impl ExactSizeIterator<Item = Fighter<'_>> {
    sol.uad
        .fits
        .get(fit_key)
        .fighters
        .iter()
        .map(|item_key| Fighter::new(sol, *item_key))
}
