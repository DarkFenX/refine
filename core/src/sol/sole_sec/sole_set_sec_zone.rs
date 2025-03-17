use crate::sol::{SolSecZone, SolarSystem};

impl SolarSystem {
    pub fn set_sec_zone(&mut self, sec_zone: SolSecZone) {
        self.uad.sec_zone = sec_zone;
    }
}
