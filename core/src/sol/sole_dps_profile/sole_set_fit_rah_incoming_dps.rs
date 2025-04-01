use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{EmDmgNonNegError, ExplDmgNonNegError, FitFoundError, KinDmgNonNegError, ThermDmgNonNegError},
    sol::{DpsProfile, FitId, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_rah_incoming_dps(
        &mut self,
        fit_id: &FitId,
        dps_profile: DpsProfile,
    ) -> Result<(), SetFitRahIncomingDpsError> {
        // Only check for negative damage values; total value of 0 is acceptable in this case
        if dps_profile.em < OF(0.0) {
            return Err(EmDmgNonNegError { value: dps_profile.em }.into());
        }
        if dps_profile.thermal < OF(0.0) {
            return Err(ThermDmgNonNegError {
                value: dps_profile.thermal,
            }
            .into());
        }
        if dps_profile.kinetic < OF(0.0) {
            return Err(KinDmgNonNegError {
                value: dps_profile.kinetic,
            }
            .into());
        }
        if dps_profile.explosive < OF(0.0) {
            return Err(ExplDmgNonNegError {
                value: dps_profile.explosive,
            }
            .into());
        }
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
    EmDpsNegative(EmDmgNonNegError),
    ThermDpsNegative(ThermDmgNonNegError),
    KinDpsNegative(KinDmgNonNegError),
    ExplDpsNegative(ExplDmgNonNegError),
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitRahIncomingDpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmDpsNegative(e) => Some(e),
            Self::ThermDpsNegative(e) => Some(e),
            Self::KinDpsNegative(e) => Some(e),
            Self::ExplDpsNegative(e) => Some(e),
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitRahIncomingDpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmDpsNegative(e) => e.fmt(f),
            Self::ThermDpsNegative(e) => e.fmt(f),
            Self::KinDpsNegative(e) => e.fmt(f),
            Self::ExplDpsNegative(e) => e.fmt(f),
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgNonNegError> for SetFitRahIncomingDpsError {
    fn from(error: EmDmgNonNegError) -> Self {
        Self::EmDpsNegative(error)
    }
}
impl From<ThermDmgNonNegError> for SetFitRahIncomingDpsError {
    fn from(error: ThermDmgNonNegError) -> Self {
        Self::ThermDpsNegative(error)
    }
}
impl From<KinDmgNonNegError> for SetFitRahIncomingDpsError {
    fn from(error: KinDmgNonNegError) -> Self {
        Self::KinDpsNegative(error)
    }
}
impl From<ExplDmgNonNegError> for SetFitRahIncomingDpsError {
    fn from(error: ExplDmgNonNegError) -> Self {
        Self::ExplDpsNegative(error)
    }
}
impl From<FitFoundError> for SetFitRahIncomingDpsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
