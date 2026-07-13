use crate::{
    api::{Fit, FitMut},
    misc::DpsProfile,
    sol::SolarSystem,
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_fit_rah_incoming_dps(&self, fit_uid: UFitId) -> Option<DpsProfile> {
        self.u_data.fits.get(fit_uid).rah_incoming_dps
    }
}

impl<'s> Fit<'s> {
    pub fn get_rah_incoming_dps(&self) -> Option<DpsProfile> {
        self.sol.internal_get_fit_rah_incoming_dps(self.uid)
    }
}

impl<'s> FitMut<'s> {
    pub fn get_rah_incoming_dps(&self) -> Option<DpsProfile> {
        self.sol.internal_get_fit_rah_incoming_dps(self.uid)
    }
}
