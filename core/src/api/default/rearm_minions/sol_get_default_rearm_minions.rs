use crate::{misc::RearmMinions, sol::SolarSystem};

impl SolarSystem {
    pub fn get_default_rearm_minions(&self) -> RearmMinions {
        self.u_data.default_rearm_minions
    }
}
