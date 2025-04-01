use crate::{
    err::basic::FitFoundError,
    sol::{DpsProfile, FitId, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_rah_incoming_dps(&self, fit_id: &FitId) -> Result<&Option<DpsProfile>, GetFitRahIncomingDpsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(&fit.rah_incoming_dps)
    }
}

#[derive(Debug)]
pub enum GetFitRahIncomingDpsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitRahIncomingDpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitRahIncomingDpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitRahIncomingDpsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
