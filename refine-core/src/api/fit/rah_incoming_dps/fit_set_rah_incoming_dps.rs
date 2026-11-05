use crate::{
    DpsProfile, Fit,
    sol::{SolCore, UndoStep},
    ud::UFitId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn set_rah_incoming_dps(&mut self, dps_profile: Option<DpsProfile>) {
        self.sol.internal_set_fit_rah_incoming_dps(self.uid, dps_profile)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_fit_rah_incoming_dps(
        &mut self,
        fit_uid: UFitId,
        dps_profile: Option<DpsProfile>,
    ) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        if u_fit.rah_incoming_dps == dps_profile {
            return;
        }
        let old_dps_profile = std::mem::replace(&mut u_fit.rah_incoming_dps, dps_profile);
        if self.undo.is_recording() {
            self.undo
                .record_dps_profile(UndoStep::FitRahIncomingDps(fit_uid), old_dps_profile);
        }
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dps if RAH profile is not set
        let default_dps_profile = self.u_data.default_incoming_dps;
        if old_dps_profile.unwrap_or(default_dps_profile) == dps_profile.unwrap_or(default_dps_profile) {
            return;
        }
        self.svc.notify_fit_rah_dps_profile_changed(&self.u_data, fit_uid);
    }
}
