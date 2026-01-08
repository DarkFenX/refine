use crate::{misc::ReloadOptionals, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_reload_optionals(&mut self, reload_optionals: ReloadOptionals) {
        self.u_data.default_reload_optionals = reload_optionals;
    }
}
