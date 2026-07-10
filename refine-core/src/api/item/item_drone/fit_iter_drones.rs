use crate::{
    api::{Drone, DroneMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone<'_>> {
        iter_drones(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone<'_>> {
        iter_drones(self.sol, self.uid)
    }
    pub fn iter_drones_mut(&mut self) -> MutIter<'_, DroneMut<'_>> {
        let drone_uids = self.sol.u_data.fits.get(self.uid).drones.iter().copied().collect();
        MutIter::new(self.sol, drone_uids)
    }
}

fn iter_drones(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Drone<'_>> {
    let drone_uids = sol.u_data.fits.get(fit_uid).drones.iter();
    drone_uids.map(|drone_uid| Drone::new(sol, *drone_uid))
}
