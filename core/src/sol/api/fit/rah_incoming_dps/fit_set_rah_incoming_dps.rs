use crate::{
    misc::DpsProfile,
    sol::{SolarSystem, api::FitMut},
    ud::UFitKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_rah_incoming_dps(&mut self, fit_key: UFitKey, dps_profile: DpsProfile) {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        if u_fit.rah_incoming_dps == Some(dps_profile) {
            return;
        }
        let old_dps_profile = u_fit.rah_incoming_dps.replace(dps_profile);
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dps if RAH profile is not set
        if old_dps_profile.is_none() && self.u_data.default_incoming_dps == dps_profile {
            return;
        }
        self.svc.notify_fit_rah_dps_profile_changed(&self.u_data, fit_key);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_rah_incoming_dps(&mut self, dps_profile: DpsProfile) {
        self.sol.internal_set_fit_rah_incoming_dps(self.key, dps_profile)
    }
}
