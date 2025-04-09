use ordered_float::OrderedFloat as OF;

use crate::{
    err::basic::{BreacherAbsDmgError, BreacherRelDmgError},
    sol::AttrVal,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherInfo {
    absolute_max: AttrVal,
    relative_max: AttrVal,
}
impl BreacherInfo {
    pub fn try_new(absolute_max: AttrVal, relative_max: AttrVal) -> Result<Self, NewBreacherInfoError> {
        if absolute_max < OF(0.0) {
            return Err(BreacherAbsDmgError { value: absolute_max }.into());
        }
        if relative_max < OF(0.0) || relative_max > OF(1.0) {
            return Err(BreacherRelDmgError { value: relative_max }.into());
        }
        Ok(Self {
            absolute_max,
            relative_max,
        })
    }
    pub fn get_absolute_max(&self) -> AttrVal {
        self.absolute_max
    }
    pub fn get_relative_max(&self) -> AttrVal {
        self.relative_max
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NewBreacherInfoError {
    #[error("{0}")]
    InvalidAbs(#[from] BreacherAbsDmgError),
    #[error("{0}")]
    InvalidRel(#[from] BreacherRelDmgError),
}
