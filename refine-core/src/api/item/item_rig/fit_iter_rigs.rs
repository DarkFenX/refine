use crate::{
    api::{Fit, FitMut, MutIter, Rig, RigMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig<'_>> {
        iter_rigs(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_rigs(&self) -> impl ExactSizeIterator<Item = Rig<'_>> {
        iter_rigs(self.sol, self.uid)
    }
    pub fn iter_rigs_mut(&mut self) -> MutIter<'_, RigMut<'_>> {
        let rig_uids = self.sol.u_data.fits.get(self.uid).rigs.iter().copied().collect();
        MutIter::new(self.sol, rig_uids)
    }
}

fn iter_rigs(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Rig<'_>> {
    let rig_uids = sol.u_data.fits.get(fit_uid).rigs.iter();
    rig_uids.map(|rig_uid| Rig::new(sol, *rig_uid))
}
