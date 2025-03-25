use crate::sol::{DmgProfile, SolarSystem};

impl SolarSystem {
    pub fn get_default_incoming_dmg(&self) -> &DmgProfile {
        &self.uad.default_incoming_dmg
    }
}
