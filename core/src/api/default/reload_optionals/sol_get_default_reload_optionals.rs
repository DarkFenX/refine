use crate::sol::SolarSystem;

impl SolarSystem {
    pub fn get_default_reload_optionals(&self) -> bool {
        self.u_data.default_reload_optionals
    }
}
