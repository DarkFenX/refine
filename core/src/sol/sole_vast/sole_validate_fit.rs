use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, SolarSystem,
        svc::vast::{ValOptions, ValResult},
    },
};

impl SolarSystem {
    pub fn validate_fit_fast(&mut self, fit_id: &FitId, options: &ValOptions) -> Result<bool, ValidateFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.validate_fit_fast_internal(fit_key, options))
    }
    pub(in crate::sol) fn validate_fit_fast_internal(&mut self, fit_key: FitKey, options: &ValOptions) -> bool {
        self.svc
            .vast
            .validate_fit_fast(&self.uad, &mut self.svc.calc, fit_key, options)
    }
    pub fn validate_fit_verbose(
        &mut self,
        fit_id: &FitId,
        options: &ValOptions,
    ) -> Result<ValResult, ValidateFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.validate_fit_verbose_internal(fit_key, options))
    }
    pub(in crate::sol) fn validate_fit_verbose_internal(&mut self, fit_key: FitKey, options: &ValOptions) -> ValResult {
        self.svc
            .vast
            .validate_fit_verbose(&self.uad, &mut self.svc.calc, fit_key, options)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValidateFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
