use crate::{
    misc::DpsProfile,
    sol::{
        SolarSystem,
        api::{Fit, FitMut},
    },
    ud::UFitKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_fit_rah_incoming_dps(&self, fit_key: UFitKey) -> Option<DpsProfile> {
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
