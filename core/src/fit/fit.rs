use crate::SolarSystem;

pub struct Fit {
    sol_sys: Option<SolarSystem>,
}
impl Fit {
    pub fn new(sol_sys: Option<SolarSystem>) -> Fit {
        Fit { sol_sys }
    }
}
