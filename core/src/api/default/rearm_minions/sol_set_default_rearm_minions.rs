use crate::sol::SolarSystem;

impl SolarSystem {
    pub fn set_default_rearm_minions(&mut self, rearm_minions: bool) {
        self.u_data.default_rearm_minions = rearm_minions;
    }
}
