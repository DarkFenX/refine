use crate::sol::{SolSecZone, SolarSystem};

impl SolarSystem {
    pub fn get_default_incoming_dmg(&self) -> &SolSecZone {
        &self.uad.sec_zone
    }
}
