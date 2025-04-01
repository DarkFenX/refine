use crate::sol::{SecZone, SolarSystem};

impl SolarSystem {
    pub fn set_sec_zone(&mut self, sec_zone: SecZone) {
        self.uad.sec_zone = sec_zone;
        self.svc.sol_sec_zone_changed(&self.uad);
    }
}
