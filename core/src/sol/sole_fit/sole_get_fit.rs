use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FitInfo},
};

impl SolarSystem {
    pub fn get_fit_info(&self, fit_id: &FitId) -> Result<FitInfo, GetFitInfoError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_info_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_info_internal(&self, fit_key: FitKey) -> FitInfo {
        let fit = self.uad.fits.get(fit_key);
        FitInfo::from_fit(&self.uad, fit)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitInfoError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
