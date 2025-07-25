use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Fleet, FleetMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn get_fleet(&self) -> Option<Fleet<'_>> {
        get_fleet(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_fleet(&self) -> Option<Fleet<'_>> {
        get_fleet(self.sol, self.key)
    }
    pub fn get_fleet_mut(&mut self) -> Option<FleetMut<'_>> {
        let u_fleet = self.sol.u_data.fits.get(self.key);
        u_fleet.fleet.map(|fleet_key| FleetMut::new(self.sol, fleet_key))
    }
}

fn get_fleet(sol: &SolarSystem, fit_key: UFitKey) -> Option<Fleet<'_>> {
    let u_fleet = sol.u_data.fits.get(fit_key);
    u_fleet.fleet.map(|fleet_key| Fleet::new(sol, fleet_key))
}
