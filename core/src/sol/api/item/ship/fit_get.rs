use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Ship, ShipMut},
};

impl<'a> Fit<'a> {
    pub fn get_ship(&'a self) -> Option<Ship<'a>> {
        get_ship(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_ship(&'a self) -> Option<Ship<'a>> {
        get_ship(self.sol, self.key)
    }
    pub fn get_ship_mut(&'a mut self) -> Option<ShipMut<'a>> {
        self.sol
            .uad
            .fits
            .get(self.key)
            .ship
            .map(|item_key| ShipMut::new(self.sol, item_key))
    }
}

fn get_ship(sol: &SolarSystem, fit_key: FitKey) -> Option<Ship> {
    sol.uad.fits.get(fit_key).ship.map(|item_key| Ship::new(sol, item_key))
}
