use crate::sol::{DpsProfile, SolarSystem};

impl SolarSystem {
    pub fn set_default_incoming_dps(&mut self, dps_profile: DpsProfile) {
        if self.uad.default_incoming_dps == dps_profile {
            return;
        }
        self.uad.default_incoming_dps = dps_profile;
        self.svc.default_incoming_dps_profile_changed(&self.uad);
    }
}
