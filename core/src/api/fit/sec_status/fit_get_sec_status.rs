use crate::{
    api::{Fit, FitMut},
    misc::FitSecStatus,
    sol::SolarSystem,
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_fit_sec_status(&self, fit_uid: UFitId) -> FitSecStatus {
        self.u_data.fits.get(fit_uid).sec_status
    }
}

impl<'a> Fit<'a> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.uid)
    }
}
