use crate::{
    api::{Fit, FitMut, Ship, ShipMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn get_ship(&self) -> Option<Ship<'_>> {
        get_ship(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_ship(&self) -> Option<Ship<'_>> {
        get_ship(self.sol, self.uid)
    }
    pub fn get_ship_mut(&mut self) -> Option<ShipMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .ship
            .map(|ship_uid| ShipMut::new(self.sol, ship_uid))
    }
}

fn get_ship(sol: &SolarSystem, fit_uid: UFitId) -> Option<Ship<'_>> {
    sol.u_data
        .fits
        .get(fit_uid)
        .ship
        .map(|ship_uid| Ship::new(sol, ship_uid))
}
