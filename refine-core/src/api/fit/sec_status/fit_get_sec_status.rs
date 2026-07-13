use crate::{
    api::{Fit, FitMut},
    num::FitSecStatus,
    sol::SolarSystem,
    ud::UFitId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_get_fit_sec_status(&self, fit_uid: UFitId) -> FitSecStatus {
        self.u_data.fits.get(fit_uid).sec_status
    }
}

impl<'s> Fit<'s> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.uid)
    }
}

impl<'s> FitMut<'s> {
    pub fn get_sec_status(&self) -> FitSecStatus {
        self.sol.internal_get_fit_sec_status(self.uid)
    }
}
