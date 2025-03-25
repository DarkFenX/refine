use crate::{
    err::basic::FitFoundError,
    sol::{DmgProfile, FitId, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_rah_incoming_dmg(&self, fit_id: &FitId) -> Result<&Option<DmgProfile>, GetFitRahIncomingDmgError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(&fit.rah_incoming_dmg)
    }
}

#[derive(Debug)]
pub enum GetFitRahIncomingDmgError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitRahIncomingDmgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitRahIncomingDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitRahIncomingDmgError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
