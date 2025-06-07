use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, MutIter, Subsystem, SubsystemMut},
};

impl<'a> Fit<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
        iter_subsystems(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
        iter_subsystems(self.sol, self.key)
    }
    pub fn iter_subsystems_mut(&mut self) -> MutIter<'_, SubsystemMut<'_>> {
        let subsystem_keys = self.sol.uad.fits.get(self.key).subsystems.iter().copied().collect();
        MutIter::new(self.sol, subsystem_keys)
    }
}

fn iter_subsystems(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
    sol.uad
        .fits
        .get(fit_key)
        .subsystems
        .iter()
        .map(|item_key| Subsystem::new(sol, *item_key))
}
