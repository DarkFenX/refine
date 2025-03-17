use crate::sol::{SolSecZone, SolarSystem};

impl SolarSystem {
    pub fn get_sec_zone(&self) -> &SolSecZone {
        &self.uad.sec_zone
    }
}
