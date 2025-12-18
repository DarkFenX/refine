use crate::{
    api::{Fit, FitMut},
    misc::FitSecStatus,
    sol::SolarSystem,
    ud::UFitKey,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_fit_sec_status(&self, fit_key: UFitKey) -> FitSecStatus {
        self.u_data.fits.get(fit_key).sec_status
    }
}

impl<'a> Fit<'a> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.key)
    }
}
