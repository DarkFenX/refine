use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{EmDmgNonNegError, ExplDmgNonNegError, KinDmgNonNegError, ThermDmgNonNegError, TotalDmgPositiveError},
    sol::{DmgProfile, SolarSystem},
};

impl SolarSystem {
    pub fn set_default_incoming_dmg(&mut self, dmg_profile: DmgProfile) -> Result<(), SetDefaultIncomingDmgError> {
        if dmg_profile.em < OF(0.0) {
            return Err(EmDmgNonNegError { value: dmg_profile.em }.into());
        }
        if dmg_profile.thermal < OF(0.0) {
            return Err(ThermDmgNonNegError {
                value: dmg_profile.thermal,
            }
            .into());
        }
        if dmg_profile.kinetic < OF(0.0) {
            return Err(KinDmgNonNegError {
                value: dmg_profile.kinetic,
            }
            .into());
        }
        if dmg_profile.explosive < OF(0.0) {
            return Err(ExplDmgNonNegError {
                value: dmg_profile.explosive,
            }
            .into());
        }
        let total = dmg_profile.em + dmg_profile.thermal + dmg_profile.kinetic + dmg_profile.explosive;
        if total <= OF(0.0) {
            return Err(TotalDmgPositiveError { value: total }.into());
        }
        if self.uad.default_incoming_dmg == dmg_profile {
            return Ok(());
        }
        self.uad.default_incoming_dmg = dmg_profile;
        self.svc.default_incoming_dmg_profile_changed(&self.uad);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetDefaultIncomingDmgError {
    EmDmgNegative(EmDmgNonNegError),
    ThermDmgNegative(ThermDmgNonNegError),
    KinDmgNegative(KinDmgNonNegError),
    ExplDmgNegative(ExplDmgNonNegError),
    TotalDmgNonPositive(TotalDmgPositiveError),
}
impl std::error::Error for SetDefaultIncomingDmgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmDmgNegative(e) => Some(e),
            Self::ThermDmgNegative(e) => Some(e),
            Self::KinDmgNegative(e) => Some(e),
            Self::ExplDmgNegative(e) => Some(e),
            Self::TotalDmgNonPositive(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetDefaultIncomingDmgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmDmgNegative(e) => e.fmt(f),
            Self::ThermDmgNegative(e) => e.fmt(f),
            Self::KinDmgNegative(e) => e.fmt(f),
            Self::ExplDmgNegative(e) => e.fmt(f),
            Self::TotalDmgNonPositive(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgNonNegError> for SetDefaultIncomingDmgError {
    fn from(error: EmDmgNonNegError) -> Self {
        Self::EmDmgNegative(error)
    }
}
impl From<ThermDmgNonNegError> for SetDefaultIncomingDmgError {
    fn from(error: ThermDmgNonNegError) -> Self {
        Self::ThermDmgNegative(error)
    }
}
impl From<KinDmgNonNegError> for SetDefaultIncomingDmgError {
    fn from(error: KinDmgNonNegError) -> Self {
        Self::KinDmgNegative(error)
    }
}
impl From<ExplDmgNonNegError> for SetDefaultIncomingDmgError {
    fn from(error: ExplDmgNonNegError) -> Self {
        Self::ExplDmgNegative(error)
    }
}
impl From<TotalDmgPositiveError> for SetDefaultIncomingDmgError {
    fn from(error: TotalDmgPositiveError) -> Self {
        Self::TotalDmgNonPositive(error)
    }
}
