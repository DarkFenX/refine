use crate::sol::{
    FitKey, SecStatus, SolarSystem,
    api::{Fit, FitMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_get_fit_sec_status(&self, fit_key: FitKey) -> SecStatus {
        self.uad.fits.get(fit_key).sec_status
    }
}

impl<'a> Fit<'a> {
    pub fn get_sec_status(&self) -> SecStatus {
        self.sol.internal_get_fit_sec_status(self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_sec_status(&self) -> SecStatus {
        self.sol.internal_get_fit_sec_status(self.key)
    }
}
