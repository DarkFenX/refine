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

#[derive(thiserror::Error, Debug)]
pub enum GetFitRahIncomingDpsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
