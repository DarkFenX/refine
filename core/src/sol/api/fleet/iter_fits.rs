use crate::sol::{
    FleetKey, SolarSystem,
    api::{Fit, Fleet, FleetMut, MutIter, mut_iter::FitMutGenerator},
};

impl<'a> Fleet<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit> {
        iter_fits(self.sol, self.key)
    }
}

impl<'a> FleetMut<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit> {
        iter_fits(self.sol, self.key)
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMutGenerator> {
        let fit_keys = self.sol.uad.fleets.get(self.key).iter_fits().copied().collect();
        MutIter::new(self.sol, fit_keys)
    }
}

fn iter_fits(sol: &SolarSystem, fleet_key: FleetKey) -> impl ExactSizeIterator<Item = Fit> {
    let uad_fleet = sol.uad.fleets.get(fleet_key);
    uad_fleet.iter_fits().map(|&fit_key| Fit::new(sol, fit_key))
}
