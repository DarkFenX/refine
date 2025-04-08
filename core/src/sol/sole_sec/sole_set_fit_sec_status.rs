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

#[derive(Debug)]
pub enum SetFitSecStatusError {
    FitNotFound(FitFoundError),
    SecStatusError(SecStatusError),
}
impl std::error::Error for SetFitSecStatusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::SecStatusError(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitSecStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::SecStatusError(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitSecStatusError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<SecStatusError> for SetFitSecStatusError {
    fn from(error: SecStatusError) -> Self {
        Self::SecStatusError(error)
    }
}

fn check_sec_status(sec_status: SecStatus) -> Result<(), SecStatusError> {
    if sec_status > OF(5.0) || sec_status < OF(-10.0) {
        return Err(SecStatusError { sec_status });
    };
    Ok(())
}
