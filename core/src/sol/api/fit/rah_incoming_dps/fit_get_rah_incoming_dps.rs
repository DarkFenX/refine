use crate::{
    misc::DpsProfile,
    sol::{
        SolarSystem,
        api::{Fit, FitMut},
    },
    uad::UadFitKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_fit_rah_incoming_dps(&self, fit_key: UadFitKey) -> Option<DpsProfile> {
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
