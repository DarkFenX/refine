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

#[derive(thiserror::Error, Debug)]
pub enum GetFitSecStatusError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
