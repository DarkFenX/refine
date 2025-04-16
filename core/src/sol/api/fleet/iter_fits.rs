use crate::sol::{
    FleetKey, SolarSystem,
    api::{Fit, Fleet, FleetMut},
};

impl<'a> Fleet<'a> {
    pub fn iter_fits(&'a self) -> impl ExactSizeIterator<Item = Fit<'a>> {
        iter_fits(self.sol, self.key)
    }
}

impl<'a> FleetMut<'a> {
    pub fn iter_fits(&'a self) -> impl ExactSizeIterator<Item = Fit<'a>> {
        iter_fits(self.sol, self.key)
    }
}

fn iter_fits(sol: &SolarSystem, fleet_key: FleetKey) -> impl ExactSizeIterator<Item = Fit> {
    let uad_fleet = sol.uad.fleets.get(fleet_key);
    uad_fleet.iter_fits().map(|&fit_key| Fit::new(sol, fit_key))
}
