use crate::{misc::ReloadOptionals, sol::SolarSystem};

impl SolarSystem {
    pub fn get_default_reload_optionals(&self) -> ReloadOptionals {
        self.u_data.default_reload_optionals
    }
}
