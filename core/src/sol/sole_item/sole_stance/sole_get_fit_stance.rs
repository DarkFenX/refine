use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{info::SolStanceInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_stance(&self, fit_id: &SolFitId) -> Result<Option<SolStanceInfo>, GetFitStanceError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(fit
            .stance
            .map(|v| SolStanceInfo::from(self.uad.items.get_item(&v).unwrap().get_stance().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitStanceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
