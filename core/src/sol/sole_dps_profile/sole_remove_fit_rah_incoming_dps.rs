use crate::{
    err::basic::{FitDpsProfileFoundError, FitFoundError},
    sol::{FitId, FitKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fit_rah_incoming_dps(&mut self, fit_id: &FitId) -> Result<(), RemoveFitRahIncomingDpsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.remove_fit_rah_incoming_dps_internal(fit_key)?)
    }
    pub(in crate::sol) fn remove_fit_rah_incoming_dps_internal(
        &mut self,
        fit_key: FitKey,
    ) -> Result<(), FitDpsProfileFoundError> {
        let fit = self.uad.fits.get_mut(fit_key);
        let old_dps_profile = fit.rah_incoming_dps.take();
        match old_dps_profile {
            Some(old_dps_profile) => {
                // Do not trigger anything in services if effectively RAH profile is not changed -
                // RAH sim uses default incoming dps if RAH profile is not set
                if self.uad.default_incoming_dps != old_dps_profile {
                    self.svc.default_incoming_dps_profile_changed(&self.uad);
                }
            }
            None => return Err(FitDpsProfileFoundError { fit_id: fit.id }.into()),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitRahIncomingDpsError {
    FitNotFound(FitFoundError),
    DpsProfileNotSet(FitDpsProfileFoundError),
}
impl std::error::Error for RemoveFitRahIncomingDpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::DpsProfileNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitRahIncomingDpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::DpsProfileNotSet(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitRahIncomingDpsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitDpsProfileFoundError> for RemoveFitRahIncomingDpsError {
    fn from(error: FitDpsProfileFoundError) -> Self {
        Self::DpsProfileNotSet(error)
    }
}
