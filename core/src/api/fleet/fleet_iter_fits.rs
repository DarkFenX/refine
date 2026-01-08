use crate::{
    api::{Fit, FitMut, Fleet, FleetMut, MutIter},
    sol::SolarSystem,
    ud::UFleetId,
};

impl<'a> Fleet<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        iter_fits(self.sol, self.uid)
    }
}

impl<'a> FleetMut<'a> {
    pub fn iter_fits(&self) -> impl ExactSizeIterator<Item = Fit<'_>> {
        iter_fits(self.sol, self.uid)
    }
    pub fn iter_fits_mut(&mut self) -> MutIter<'_, FitMut<'_>> {
        let fit_uids = self.sol.u_data.fleets.get(self.uid).iter_fits().collect();
        MutIter::new(self.sol, fit_uids)
    }
}

fn iter_fits(sol: &SolarSystem, fleet_uid: UFleetId) -> impl ExactSizeIterator<Item = Fit<'_>> {
    let u_fleet = sol.u_data.fleets.get(fleet_uid);
    u_fleet.iter_fits().map(|fit_uid| Fit::new(sol, fit_uid))
}
