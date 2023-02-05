use crate::{Ship, SolarSystem};

pub struct Fit {
    sol_sys: Option<SolarSystem>,
    ship: Option<Ship>,
}
impl Fit {
    pub fn new(sol_sys: Option<SolarSystem>) -> Fit {
        Fit { sol_sys, ship: None }
    }
    pub fn get_ship(&self) -> Option<&Ship> {
        self.ship.as_ref()
    }
    pub fn set_ship(&mut self, ship: Option<Ship>) {
        self.ship = ship
    }
}
