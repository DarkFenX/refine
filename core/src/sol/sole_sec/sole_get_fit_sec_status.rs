use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SecStatus, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_sec_status(&self, fit_id: &FitId) -> Result<SecStatus, GetFitSecStatusError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(fit.sec_status)
    }
}

#[derive(Debug)]
pub enum GetFitSecStatusError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitSecStatusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitSecStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitSecStatusError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
