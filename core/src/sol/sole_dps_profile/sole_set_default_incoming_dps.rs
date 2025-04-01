use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{EmDmgNonNegError, ExplDmgNonNegError, KinDmgNonNegError, ThermDmgNonNegError, TotalDmgPositiveError},
    sol::{DpsProfile, SolarSystem},
};

impl SolarSystem {
    pub fn set_default_incoming_dps(&mut self, dps_profile: DpsProfile) -> Result<(), SetDefaultIncomingDpsError> {
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
        let total = dps_profile.em + dps_profile.thermal + dps_profile.kinetic + dps_profile.explosive;
        if total <= OF(0.0) {
            return Err(TotalDmgPositiveError { value: total }.into());
        }
        if self.uad.default_incoming_dps == dps_profile {
            return Ok(());
        }
        self.uad.default_incoming_dps = dps_profile;
        self.svc.default_incoming_dps_profile_changed(&self.uad);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetDefaultIncomingDpsError {
    EmDpsNegative(EmDmgNonNegError),
    ThermDpsNegative(ThermDmgNonNegError),
    KinDpsNegative(KinDmgNonNegError),
    ExplDpsNegative(ExplDmgNonNegError),
    TotalDpsNonPositive(TotalDmgPositiveError),
}
impl std::error::Error for SetDefaultIncomingDpsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmDpsNegative(e) => Some(e),
            Self::ThermDpsNegative(e) => Some(e),
            Self::KinDpsNegative(e) => Some(e),
            Self::ExplDpsNegative(e) => Some(e),
            Self::TotalDpsNonPositive(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetDefaultIncomingDpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmDpsNegative(e) => e.fmt(f),
            Self::ThermDpsNegative(e) => e.fmt(f),
            Self::KinDpsNegative(e) => e.fmt(f),
            Self::ExplDpsNegative(e) => e.fmt(f),
            Self::TotalDpsNonPositive(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgNonNegError> for SetDefaultIncomingDpsError {
    fn from(error: EmDmgNonNegError) -> Self {
        Self::EmDpsNegative(error)
    }
}
impl From<ThermDmgNonNegError> for SetDefaultIncomingDpsError {
    fn from(error: ThermDmgNonNegError) -> Self {
        Self::ThermDpsNegative(error)
    }
}
impl From<KinDmgNonNegError> for SetDefaultIncomingDpsError {
    fn from(error: KinDmgNonNegError) -> Self {
        Self::KinDpsNegative(error)
    }
}
impl From<ExplDmgNonNegError> for SetDefaultIncomingDpsError {
    fn from(error: ExplDmgNonNegError) -> Self {
        Self::ExplDpsNegative(error)
    }
}
impl From<TotalDmgPositiveError> for SetDefaultIncomingDpsError {
    fn from(error: TotalDmgPositiveError) -> Self {
        Self::TotalDpsNonPositive(error)
    }
}
