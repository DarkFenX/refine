use crate::{SecZone, SolarSystem};

impl SolarSystem {
    pub fn get_sec_zone(&self) -> SecZone {
        self.u_data.sec_zone
    }
}
