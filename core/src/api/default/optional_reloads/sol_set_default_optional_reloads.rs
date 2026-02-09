use crate::{misc::OptionalReload, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_optional_reloads(&mut self, optional_reloads: OptionalReload) {
        self.u_data.default_optional_reloads = optional_reloads;
    }
}
