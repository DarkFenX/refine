use crate::sol::{SolDmgProfile, SolarSystem};

impl SolarSystem {
    pub fn get_default_incoming_dmg(&self) -> &SolDmgProfile {
        &self.uad.default_incoming_dmg
    }
}
