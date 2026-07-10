use crate::{misc::RearmMinion, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_rearm_minions(&mut self, rearm_minions: RearmMinion) {
        self.u_data.default_rearm_minions = rearm_minions;
    }
}
