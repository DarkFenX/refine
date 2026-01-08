use crate::{misc::RearmMinions, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_rearm_minions(&mut self, rearm_minions: RearmMinions) {
        self.u_data.default_rearm_minions = rearm_minions;
    }
}
