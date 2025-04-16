use crate::sol::{
    FitKey, SolarSystem,
    api::{Drone, Fit, FitMut},
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
}

fn iter_drones(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Drone> {
    sol.uad
        .fits
        .get(fit_key)
        .drones
        .iter()
        .map(|item_key| Drone::new(sol, *item_key))
}
