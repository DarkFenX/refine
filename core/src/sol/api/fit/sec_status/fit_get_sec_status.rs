use crate::{
    misc::FitSecStatus,
    sol::{
        SolarSystem,
        api::{Fit, FitMut},
    },
    ud::UFitKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_fit_sec_status(&self, fit_key: UFitKey) -> FitSecStatus {
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
