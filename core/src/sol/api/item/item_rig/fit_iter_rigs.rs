use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, MutIter, Rig, RigMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig<'_>> {
        iter_rigs(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig<'_>> {
        iter_rigs(self.sol, self.key)
    }
    pub fn iter_rigs_mut(&mut self) -> MutIter<'_, RigMut<'_>> {
        let implant_keys = self.sol.u_data.fits.get(self.key).rigs.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_rigs(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Rig<'_>> {
    sol.u_data
        .fits
        .get(fit_key)
        .rigs
        .iter()
        .map(|item_key| Rig::new(sol, *item_key))
}
