use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{FitFoundError, SecStatusError},
    sol::{FitId, FitKey, SecStatus, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_sec_status(&mut self, fit_id: &FitId, sec_status: SecStatus) -> Result<(), SetFitSecStatusError> {
        check_sec_status(sec_status)?;
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        self.set_fit_sec_status_internal(fit_key, sec_status);
        Ok(())
    }
    pub(in crate::sol) fn set_fit_sec_status_internal(&mut self, fit_key: FitKey, sec_status: SecStatus) {
        let fit = self.uad.fits.get_mut(fit_key);
        let old_sec_status = fit.sec_status;
        if old_sec_status == sec_status {
            return;
        }
        fit.sec_status = sec_status;
        if let Some(ship_key) = fit.ship {
            self.svc.ship_sec_status_changed(&self.uad, ship_key);
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitSecStatusError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    SecStatusError(#[from] SecStatusError),
}

fn check_sec_status(sec_status: SecStatus) -> Result<(), SecStatusError> {
    if sec_status > OF(5.0) || sec_status < OF(-10.0) {
        return Err(SecStatusError { sec_status });
    };
    Ok(())
}
