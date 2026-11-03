use crate::{DpsProfile, SolarSystem, sol::UndoStep};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn set_default_incoming_dps(&mut self, dps_profile: DpsProfile) {
        // Update user data
        let old_dps_profile = std::mem::replace(&mut self.u_data.default_incoming_dps, dps_profile);
        if old_dps_profile == dps_profile {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record_dps_profile(UndoStep::DefaultIncomingDps, Some(old_dps_profile));
        }
        // Update services
        for (fit_uid, fit) in self.u_data.fits.iter() {
            if fit.rah_incoming_dps.is_none() {
                self.svc.notify_fit_rah_dps_profile_changed(&self.u_data, fit_uid);
            }
        }
    }
}
