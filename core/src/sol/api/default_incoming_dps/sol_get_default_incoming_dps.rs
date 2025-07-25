use crate::{misc::DpsProfile, sol::SolarSystem};

impl SolarSystem {
    pub fn get_default_incoming_dps(&self) -> DpsProfile {
        self.u_data.default_incoming_dps
    }
}
