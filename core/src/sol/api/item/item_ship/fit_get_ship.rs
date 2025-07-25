use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Ship, ShipMut},
    },
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn get_ship(&self) -> Option<Ship<'_>> {
        get_ship(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_ship(&self) -> Option<Ship<'_>> {
        get_ship(self.sol, self.key)
    }
    pub fn get_ship_mut(&mut self) -> Option<ShipMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.key)
            .ship
            .map(|item_key| ShipMut::new(self.sol, item_key))
    }
}

fn get_ship(sol: &SolarSystem, fit_key: UFitKey) -> Option<Ship<'_>> {
    sol.u_data
        .fits
        .get(fit_key)
        .ship
        .map(|item_key| Ship::new(sol, item_key))
}
