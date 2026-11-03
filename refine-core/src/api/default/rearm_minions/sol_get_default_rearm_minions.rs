use crate::{RearmMinion, SolarSystem};

impl SolarSystem {
    pub fn get_default_rearm_minions(&self) -> RearmMinion {
        self.u_data.default_rearm_minions
    }
}
