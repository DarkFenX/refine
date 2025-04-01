use crate::{
    err::basic::{FitDpsProfileFoundError, FitFoundError},
    sol::{FitId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fit_rah_incoming_dps(&mut self, fit_id: &FitId) -> Result<(), RemoveFitRahIncomingDpsError> {
        let fit = self.uad.fits.get_fit_mut(fit_id)?;
        let old_dps_profile = fit.rah_incoming_dps.take();
        match old_dps_profile {
            Some(old_dps_profile) => {
                // Do not trigger anything in services if effectively RAH profile is not changed -
                // RAH sim uses default incoming dps if RAH profile is not set
                if self.uad.default_incoming_dps != old_dps_profile {
                    self.svc.default_incoming_dps_profile_changed(&self.uad);
                }
            }
            None => return Err(FitDpsProfileFoundError { fit_id: *fit_id }.into()),
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
