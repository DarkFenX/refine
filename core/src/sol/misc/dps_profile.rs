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

#[derive(thiserror::Error, Debug)]
pub enum NewDpsProfileError {
    #[error("{0}")]
    InvalidEm(#[from] EmDmgError),
    #[error("{0}")]
    InvalidThermal(#[from] ThermDmgError),
    #[error("{0}")]
    InvalidKinetic(#[from] KinDmgError),
    #[error("{0}")]
    InvalidExplosive(#[from] ExplDmgError),
}
