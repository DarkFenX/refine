use crate::sol::{SecZone, SolarSystem};

impl SolarSystem {
    pub fn get_sec_zone(&self) -> &SecZone {
        &self.uad.sec_zone
    }
}
