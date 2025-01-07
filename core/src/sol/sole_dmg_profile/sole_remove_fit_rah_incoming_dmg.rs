use crate::{
    defs::SolFitId,
    err::basic::{FitDmgProfileFoundError, FitFoundError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_fit_rah_incoming_dmg(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitRahIncomingDmgError> {
        let fit = self.uad.fits.get_fit_mut(fit_id)?;
        let old_dmg_profile = fit.rah_incoming_dmg.take();
        match old_dmg_profile {
            Some(old_dmg_profile) => {
                // Do not trigger anything in services if effectively RAH profile is not changed -
                // RAH sim uses default incoming dmg if RAH profile is not set
                if self.uad.default_incoming_dmg != old_dmg_profile {
                    self.svc.default_incoming_dmg_profile_changed(&self.uad);
                }
            }
            None => return Err(FitDmgProfileFoundError::new(*fit_id).into()),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitRahIncomingDmgError {
    FitNotFound(FitFoundError),
    DmgProfileNotSet(FitDmgProfileFoundError),
}
impl std::error::Error for RemoveFitRahIncomingDmgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::DmgProfileNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitRahIncomingDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::DmgProfileNotSet(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitRahIncomingDmgError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitDmgProfileFoundError> for RemoveFitRahIncomingDmgError {
    fn from(error: FitDmgProfileFoundError) -> Self {
        Self::DmgProfileNotSet(error)
    }
}
