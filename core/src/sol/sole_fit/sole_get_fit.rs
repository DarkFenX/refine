use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{SolarSystem, info::SolFitInfo},
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &SolFitId) -> Result<SolFitInfo, GetFitError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(SolFitInfo::from(fit))
    }
}

#[derive(Debug)]
pub enum GetFitError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
