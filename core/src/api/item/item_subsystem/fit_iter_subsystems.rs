use crate::{
    api::{Fit, FitMut, MutIter, Subsystem, SubsystemMut},
    sol::SolarSystem,
    ud::UFitKey,
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
        let subsystem_keys = self.sol.u_data.fits.get(self.key).subsystems.iter().copied().collect();
        MutIter::new(self.sol, subsystem_keys)
    }
}

fn iter_subsystems(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Subsystem<'_>> {
    let subsystem_keys = sol.u_data.fits.get(fit_key).subsystems.iter();
    subsystem_keys.map(|subsystem_key| Subsystem::new(sol, *subsystem_key))
}
