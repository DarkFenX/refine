use crate::sol::{
    DpsProfile, FitKey, SolarSystem,
    api::{Fit, FitMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_get_fit_rah_incoming_dps(&self, fit_key: FitKey) -> Option<DpsProfile> {
        self.uad.fits.get(fit_key).rah_incoming_dps
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
