use crate::sol::{SolSecZone, SolarSystem};

impl SolarSystem {
    pub fn set_default_incoming_dmg(&mut self, sec_zone: SolSecZone) {
        self.uad.sec_zone = sec_zone;
    }
}
