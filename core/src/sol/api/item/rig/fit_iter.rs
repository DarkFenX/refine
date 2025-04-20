use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, ItemMutIter, Rig, RigMutGenerator},
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
    pub fn iter_rigs_mut(&mut self) -> ItemMutIter<'_, RigMutGenerator> {
        let implant_keys = self.sol.uad.fits.get(self.key).rigs.iter().copied().collect();
        ItemMutIter::new(self.sol, implant_keys)
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
