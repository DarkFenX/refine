use crate::{
    api::{Fit, FitMut, Fleet, FleetMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn get_fleet(&self) -> Option<Fleet<'_>> {
        get_fleet(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_fleet(&self) -> Option<Fleet<'_>> {
        get_fleet(self.sol, self.uid)
    }
    pub fn get_fleet_mut(&mut self) -> Option<FleetMut<'_>> {
        let u_fleet = self.sol.u_data.fits.get(self.uid);
        u_fleet.fleet.map(|fleet_uid| FleetMut::new(self.sol, fleet_uid))
    }
}

fn get_fleet(sol: &SolarSystem, fit_uid: UFitId) -> Option<Fleet<'_>> {
    let u_fleet = sol.u_data.fits.get(fit_uid);
    u_fleet.fleet.map(|fleet_uid| Fleet::new(sol, fleet_uid))
}
