use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SecStatus, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_sec_status(&self, fit_id: &FitId) -> Result<SecStatus, GetFitSecStatusError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_sec_status_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_sec_status_internal(&self, fit_key: FitKey) -> SecStatus {
        self.uad.fits.get(fit_key).sec_status
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
