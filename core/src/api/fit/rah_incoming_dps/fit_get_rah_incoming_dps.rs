use crate::{
    api::{Fit, FitMut},
    misc::DpsProfile,
    sol::SolarSystem,
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_fit_rah_incoming_dps(&self, fit_key: UFitId) -> Option<DpsProfile> {
        self.u_data.fits.get(fit_key).rah_incoming_dps
    }
}

impl<'a> Fit<'a> {
    pub fn get_rah_incoming_dps(&self) -> Option<DpsProfile> {
        self.sol.internal_get_fit_rah_incoming_dps(self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_rah_incoming_dps(&self) -> Option<DpsProfile> {
        self.sol.internal_get_fit_rah_incoming_dps(self.key)
    }
}
