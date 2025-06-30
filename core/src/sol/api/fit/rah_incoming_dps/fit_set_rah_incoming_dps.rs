use crate::{
    def::FitKey,
    misc::DpsProfile,
    sol::{SolarSystem, api::FitMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_rah_incoming_dps(&mut self, fit_key: FitKey, dps_profile: DpsProfile) {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        if uad_fit.rah_incoming_dps == Some(dps_profile) {
            return;
        }
        let old_dps_profile = uad_fit.rah_incoming_dps.replace(dps_profile);
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dps if RAH profile is not set
        if old_dps_profile.is_none() && self.uad.default_incoming_dps == dps_profile {
            return;
        }
        self.svc.notify_fit_rah_dps_profile_changed(&self.uad, &fit_key);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_rah_incoming_dps(&mut self, dps_profile: DpsProfile) {
        self.sol.internal_set_fit_rah_incoming_dps(self.key, dps_profile)
    }
}
