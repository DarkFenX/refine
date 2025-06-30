use crate::{
    def::FitKey,
    sol::{
        SolarSystem,
        api::{Drone, DroneMut, Fit, FitMut, MutIter},
    },
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
        let implant_keys = self.sol.uad.fits.get(self.key).drones.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_drones(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Drone<'_>> {
    sol.uad
        .fits
        .get(fit_key)
        .drones
        .iter()
        .map(|item_key| Drone::new(sol, *item_key))
}
