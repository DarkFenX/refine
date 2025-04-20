use crate::sol::{
    FitKey, SolarSystem,
    api::{Drone, DroneMutGenerator, Fit, FitMut, ItemMutIter},
};

impl<'a> Fit<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone> {
        iter_drones(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_drones(&self) -> impl ExactSizeIterator<Item = Drone> {
        iter_drones(self.sol, self.key)
    }
    pub fn iter_drones_mut(&mut self) -> ItemMutIter<'_, DroneMutGenerator> {
        let implant_keys = self.sol.uad.fits.get(self.key).drones.iter().copied().collect();
        ItemMutIter::new(self.sol, implant_keys)
    }
}

fn iter_drones(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Drone> {
    sol.uad
        .fits
        .get(fit_key)
        .drones
        .iter()
        .map(|item_key| Drone::new(sol, *item_key))
}
