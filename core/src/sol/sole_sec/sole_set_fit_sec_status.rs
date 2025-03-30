use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{FitFoundError, SecStatusError},
    sol::{FitId, SecStatus, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_sec_status(&mut self, fit_id: &FitId, sec_status: SecStatus) -> Result<(), SetFitSecStatusError> {
        check_sec_status(sec_status)?;
        let fit = self.uad.fits.get_fit_mut(fit_id)?;
        let old_sec_status = fit.sec_status;
        if old_sec_status == sec_status {
            return Ok(());
        }
        fit.sec_status = sec_status;
        if let Some(ship_id) = fit.ship {
            let ship = self.uad.items.get_item(&ship_id).unwrap().get_ship().unwrap();
            self.svc.ship_sec_status_changed(&self.uad, ship);
        }
        Ok(())
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
