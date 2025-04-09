use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FitInfo},
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &FitId) -> Result<FitInfo, GetFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_internal(&self, fit_key: FitKey) -> FitInfo {
        let fit = self.uad.fits.get(fit_key);
        FitInfo::from_fit(&self.uad, fit)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
