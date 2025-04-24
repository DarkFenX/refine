use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::SecStatusError,
    sol::{FitKey, SecStatus, SolarSystem, api::FitMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_fit_sec_status(&mut self, fit_key: FitKey, sec_status: SecStatus) {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let old_sec_status = uad_fit.sec_status;
        if old_sec_status == sec_status {
            return;
        }
        uad_fit.sec_status = sec_status;
        if let Some(ship_key) = uad_fit.ship {
            self.svc.ship_sec_status_changed(&self.uad, ship_key);
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn set_sec_status(&mut self, sec_status: SecStatus) -> Result<(), SetFitSecStatusError> {
        check_sec_status(sec_status)?;
        self.sol.internal_set_fit_sec_status(self.key, sec_status);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitSecStatusError {
    #[error("{0}")]
    SecStatusError(#[from] SecStatusError),
}

fn check_sec_status(sec_status: SecStatus) -> Result<(), SecStatusError> {
    match sec_status > OF(5.0) || sec_status < OF(-10.0) {
        true => Err(SecStatusError { sec_status }),
        false => Ok(()),
    }
}
