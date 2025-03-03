use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{
        SolarSystem,
        svc::vast::{SolValOptions, SolValResult},
    },
};

impl SolarSystem {
    pub fn validate_fit_fast(&mut self, fit_id: &SolFitId, options: SolValOptions) -> Result<bool, ValidateFitError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(self
            .svc
            .vast
            .validate_fit_fast(&self.uad, &mut self.svc.calc, fit, options))
    }
    pub fn validate_fit_verbose(
        &mut self,
        fit_id: &SolFitId,
        options: SolValOptions,
    ) -> Result<SolValResult, ValidateFitError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(self
            .svc
            .vast
            .validate_fit_verbose(&self.uad, &mut self.svc.calc, fit, options))
    }
}

#[derive(Debug)]
pub enum ValidateFitError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for ValidateFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ValidateFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for ValidateFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
