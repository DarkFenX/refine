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
        let rig_keys = self.sol.u_data.fits.get(self.key).rigs.iter().copied().collect();
        MutIter::new(self.sol, rig_keys)
    }
}

fn iter_rigs(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Rig<'_>> {
    let rig_keys = sol.u_data.fits.get(fit_key).rigs.iter();
    rig_keys.map(|rig_key| Rig::new(sol, *rig_key))
}
