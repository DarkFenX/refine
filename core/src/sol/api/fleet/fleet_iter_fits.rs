use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Fleet, FleetMut, MutIter},
    },
    ud::UFleetKey,
};

impl<'a> Fleet<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        iter_fits(self.sol, self.key)
    }
}

impl<'a> FleetMut<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        iter_fits(self.sol, self.key)
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMut<'_>> {
        let fit_keys = self.sol.u_data.fleets.get(self.key).iter_fits().collect();
        MutIter::new(self.sol, fit_keys)
    }
}

fn iter_fits(sol: &SolarSystem, fleet_key: UFleetKey) -> impl ExactSizeIterator<Item = Fit<'_>> {
    let u_fleet = sol.u_data.fleets.get(fleet_key);
    u_fleet.iter_fits().map(|fit_key| Fit::new(sol, fit_key))
}
