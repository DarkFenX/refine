use crate::{Ship, SolarSystem};

pub struct Fit {
    sol_sys: Option<SolarSystem>,
    ship: Option<Ship>,
}
impl Fit {
    pub fn new(sol_sys: Option<SolarSystem>) -> Fit {
        Fit { sol_sys, ship: None }
    }
}
