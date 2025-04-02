use crate::{
    err::basic::FitFoundError,
    sol::{DpsProfile, FitId, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_rah_incoming_dps(
        &mut self,
        fit_id: &FitId,
        dps_profile: DpsProfile,
    ) -> Result<(), SetFitRahIncomingDpsError> {
        let fit = self.uad.fits.get_fit_mut(fit_id)?;
        if fit.rah_incoming_dps == Some(dps_profile) {
            return Ok(());
        }
        let old_dps_profile = fit.rah_incoming_dps.replace(dps_profile);
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dps if RAH profile is not set
        if old_dps_profile.is_none() && self.uad.default_incoming_dps == dps_profile {
            return Ok(());
        }
        self.svc.fit_rah_dps_profile_changed(&self.uad, fit_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitRahIncomingDpsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitRahIncomingDpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitRahIncomingDpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitRahIncomingDpsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
