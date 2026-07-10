use crate::{
    api::{Fit, FitMut, MutIter, Subsystem, SubsystemMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
        iter_subsystems(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_subsystems(&self) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
        iter_subsystems(self.sol, self.uid)
    }
    pub fn iter_subsystems_mut(&mut self) -> MutIter<'_, SubsystemMut<'_>> {
        let subsystem_uids = self.sol.u_data.fits.get(self.uid).subsystems.iter().copied().collect();
        MutIter::new(self.sol, subsystem_uids)
    }
}

fn iter_subsystems(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
    let subsystem_uids = sol.u_data.fits.get(fit_uid).subsystems.iter();
    subsystem_uids.map(|subsystem_uid| Subsystem::new(sol, *subsystem_uid))
}
