use crate::{
    api::{Drone, DroneMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone<'_>> {
        iter_drones(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone<'_>> {
        iter_drones(self.sol, self.key)
    }
    pub fn iter_drones_mut(&mut self) -> MutIter<'_, DroneMut<'_>> {
        let implant_keys = self.sol.u_data.fits.get(self.key).drones.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_drones(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = Drone<'_>> {
    let drone_keys = sol.u_data.fits.get(fit_key).drones.iter();
    drone_keys.map(|drone_key| Drone::new(sol, *drone_key))
}
