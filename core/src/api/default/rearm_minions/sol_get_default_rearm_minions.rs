use crate::sol::SolarSystem;

impl SolarSystem {
    pub fn get_default_rearm_minions(&self) -> bool {
        self.u_data.default_rearm_minions
    }
}
