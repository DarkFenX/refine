use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, SolarSystem,
        api::{Fit, FitMut},
    },
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &FitId) -> Result<Fit, GetFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.internal_get_fit(fit_key))
    }
    pub fn get_fit_mut(&mut self, fit_id: &FitId) -> Result<FitMut, GetFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.internal_get_fit_mut(fit_key))
    }
    pub(in crate::sol) fn internal_get_fit(&self, fit_key: FitKey) -> Fit {
        Fit::new(self, fit_key)
    }
    pub(in crate::sol) fn internal_get_fit_mut(&mut self, fit_key: FitKey) -> FitMut {
        FitMut::new(self, fit_key)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
