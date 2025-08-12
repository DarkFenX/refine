use crate::{misc::DpsProfile, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_incoming_dps(&mut self, dps_profile: DpsProfile) {
        if self.u_data.default_incoming_dps == dps_profile {
            return;
        }
        // Update user data
        self.u_data.default_incoming_dps = dps_profile;
        // Update services
        for (fit_key, fit) in self.u_data.fits.iter() {
            if fit.rah_incoming_dps.is_none() {
                self.svc.notify_fit_rah_dps_profile_changed(&self.u_data, fit_key);
            }
        }
    }
}
