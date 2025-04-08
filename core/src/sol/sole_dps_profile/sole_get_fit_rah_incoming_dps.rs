use crate::{
    err::basic::FitFoundError,
    sol::{DpsProfile, FitId, FitKey, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_rah_incoming_dps(&self, fit_id: &FitId) -> Result<Option<DpsProfile>, GetFitRahIncomingDpsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_rah_incoming_dps_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_rah_incoming_dps_internal(&self, fit_key: FitKey) -> Option<DpsProfile> {
        self.uad.fits.get(fit_key).rah_incoming_dps
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
