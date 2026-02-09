use crate::{misc::OptionalReload, sol::SolarSystem};

impl SolarSystem {
    pub fn get_default_optional_reloads(&self) -> OptionalReload {
        self.u_data.default_optional_reloads
    }
}
