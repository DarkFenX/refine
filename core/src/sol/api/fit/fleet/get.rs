use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Fleet, FleetMut},
};

impl<'a> Fit<'a> {
    pub fn get_fleet(&self) -> Option<Fleet> {
        get_fleet(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_fleet(&self) -> Option<Fleet> {
        get_fleet(self.sol, self.key)
    }
    pub fn get_fleet_mut(&mut self) -> Option<FleetMut> {
        let uad_fleet = self.sol.uad.fits.get(self.key);
        uad_fleet.fleet.map(|fleet_key| FleetMut::new(self.sol, fleet_key))
    }
}

fn get_fleet(sol: &SolarSystem, fit_key: FitKey) -> Option<Fleet> {
    let uad_fleet = sol.uad.fits.get(fit_key);
    uad_fleet.fleet.map(|fleet_key| Fleet::new(sol, fleet_key))
}
