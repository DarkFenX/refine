use crate::sol::{SolDmgProfile, SolarSystem};

impl SolarSystem {
    pub fn get_default_incoming_dmg(&self) -> &SolDmgProfile {
        &self.default_incoming_dmg
    }
}
