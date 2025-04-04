use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{EmDmgError, ExplDmgError, KinDmgError, ThermDmgError},
    sol::{AttrVal, BreacherInfo},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    em: AttrVal,
    thermal: AttrVal,
    kinetic: AttrVal,
    explosive: AttrVal,
    breacher: Option<BreacherInfo>,
}
impl DpsProfile {
    pub fn try_new(
        em: AttrVal,
        thermal: AttrVal,
        kinetic: AttrVal,
        explosive: AttrVal,
        breacher: Option<BreacherInfo>,
    ) -> Result<Self, NewDpsProfileError> {
        if em < OF(0.0) {
            return Err(EmDmgError { value: em }.into());
        }
        if thermal < OF(0.0) {
            return Err(ThermDmgError { value: thermal }.into());
        }
        if kinetic < OF(0.0) {
            return Err(KinDmgError { value: kinetic }.into());
        }
        if explosive < OF(0.0) {
            return Err(ExplDmgError { value: explosive }.into());
        }
        Ok(Self {
            em,
            thermal,
            kinetic,
            explosive,
            breacher,
        })
    }
    pub fn get_em(&self) -> AttrVal {
        self.em
    }
    pub fn get_thermal(&self) -> AttrVal {
        self.thermal
    }
    pub fn get_kinetic(&self) -> AttrVal {
        self.kinetic
    }
    pub fn get_explosive(&self) -> AttrVal {
        self.explosive
    }
    pub fn get_breacher(&self) -> Option<BreacherInfo> {
        self.breacher
    }
    pub(in crate::sol) fn deals_breacher_dps(&self) -> bool {
        match self.breacher {
            Some(breacher) => breacher.get_relative_max() > OF(0.0) && breacher.get_absolute_max() > OF(0.0),
            None => false,
        }
    }
}
#[derive(Debug)]
pub enum NewDpsProfileError {
    InvalidEm(EmDmgError),
    InvalidThermal(ThermDmgError),
    InvalidKinetic(KinDmgError),
    InvalidExplosive(ExplDmgError),
}
impl std::error::Error for NewDpsProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidEm(e) => Some(e),
            Self::InvalidThermal(e) => Some(e),
            Self::InvalidKinetic(e) => Some(e),
            Self::InvalidExplosive(e) => Some(e),
        }
    }
}
impl std::fmt::Display for NewDpsProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidEm(e) => e.fmt(f),
            Self::InvalidThermal(e) => e.fmt(f),
            Self::InvalidKinetic(e) => e.fmt(f),
            Self::InvalidExplosive(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgError> for NewDpsProfileError {
    fn from(error: EmDmgError) -> Self {
        Self::InvalidEm(error)
    }
}
impl From<ThermDmgError> for NewDpsProfileError {
    fn from(error: ThermDmgError) -> Self {
        Self::InvalidThermal(error)
    }
}
impl From<KinDmgError> for NewDpsProfileError {
    fn from(error: KinDmgError) -> Self {
        Self::InvalidKinetic(error)
    }
}
impl From<ExplDmgError> for NewDpsProfileError {
    fn from(error: ExplDmgError) -> Self {
        Self::InvalidExplosive(error)
    }
}
