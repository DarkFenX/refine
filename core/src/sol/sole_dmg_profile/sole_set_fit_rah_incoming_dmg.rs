use crate::{
    defs::SolFitId,
    err::basic::{EmDmgNonNegError, ExplDmgNonNegError, FitFoundError, KinDmgNonNegError, ThermDmgNonNegError},
    sol::{SolDmgProfile, SolView, SolarSystem},
    OF,
};

impl SolarSystem {
    pub fn set_fit_rah_incoming_dmg(
        &mut self,
        fit_id: &SolFitId,
        dmg_profile: SolDmgProfile,
    ) -> Result<(), SetFitRahIncomingDmgError> {
        // Only check for negative damage values; total value of 0 is acceptable in this case
        if dmg_profile.em < OF(0.0) {
            return Err(EmDmgNonNegError::new(dmg_profile.em).into());
        }
        if dmg_profile.thermal < OF(0.0) {
            return Err(ThermDmgNonNegError::new(dmg_profile.thermal).into());
        }
        if dmg_profile.kinetic < OF(0.0) {
            return Err(KinDmgNonNegError::new(dmg_profile.kinetic).into());
        }
        if dmg_profile.explosive < OF(0.0) {
            return Err(ExplDmgNonNegError::new(dmg_profile.explosive).into());
        }
        let fit = self.fits.get_fit_mut(fit_id)?;
        if fit.rah_incoming_dmg == Some(dmg_profile) {
            return Ok(());
        }
        let old_dmg_profile = fit.rah_incoming_dmg.replace(dmg_profile);
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dmg if RAH profile is not set
        if old_dmg_profile.is_none() && self.default_incoming_dmg == dmg_profile {
            return Ok(());
        }
        self.svcs.fit_rah_dmg_profile_changed(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            fit_id,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitRahIncomingDmgError {
    EmDmgNegative(EmDmgNonNegError),
    ThermDmgNegative(ThermDmgNonNegError),
    KinDmgNegative(KinDmgNonNegError),
    ExplDmgNegative(ExplDmgNonNegError),
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitRahIncomingDmgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmDmgNegative(e) => Some(e),
            Self::ThermDmgNegative(e) => Some(e),
            Self::KinDmgNegative(e) => Some(e),
            Self::ExplDmgNegative(e) => Some(e),
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitRahIncomingDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmDmgNegative(e) => e.fmt(f),
            Self::ThermDmgNegative(e) => e.fmt(f),
            Self::KinDmgNegative(e) => e.fmt(f),
            Self::ExplDmgNegative(e) => e.fmt(f),
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgNonNegError> for SetFitRahIncomingDmgError {
    fn from(error: EmDmgNonNegError) -> Self {
        Self::EmDmgNegative(error)
    }
}
impl From<ThermDmgNonNegError> for SetFitRahIncomingDmgError {
    fn from(error: ThermDmgNonNegError) -> Self {
        Self::ThermDmgNegative(error)
    }
}
impl From<KinDmgNonNegError> for SetFitRahIncomingDmgError {
    fn from(error: KinDmgNonNegError) -> Self {
        Self::KinDmgNegative(error)
    }
}
impl From<ExplDmgNonNegError> for SetFitRahIncomingDmgError {
    fn from(error: ExplDmgNonNegError) -> Self {
        Self::ExplDmgNegative(error)
    }
}
impl From<FitFoundError> for SetFitRahIncomingDmgError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
