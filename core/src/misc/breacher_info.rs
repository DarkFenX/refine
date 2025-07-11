use crate::{
    def::{AttrVal, OF},
    err::basic::BreacherDmgError,
};

const ABS_MIN: AttrVal = OF(0.0);
const REL_MIN: AttrVal = OF(0.0);
const REL_MAX: AttrVal = OF(1.0);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherInfo {
    absolute_max: AttrVal,
    relative_max: AttrVal,
}
impl BreacherInfo {
    pub fn new_clamped(absolute_max: AttrVal, relative_max: AttrVal) -> Self {
        Self {
            absolute_max: AttrVal::max(absolute_max, ABS_MIN),
            relative_max: AttrVal::max(AttrVal::min(relative_max, REL_MAX), REL_MIN),
        }
    }
    pub fn try_new(absolute_max: AttrVal, relative_max: AttrVal) -> Result<Self, BreacherInfoError> {
        if absolute_max < ABS_MIN {
            return Err(BreacherDmgError::Absolute(absolute_max).into());
        }
        if relative_max < REL_MIN || relative_max > REL_MAX {
            return Err(BreacherDmgError::Absolute(relative_max).into());
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
pub enum BreacherInfoError {
    #[error("{0}")]
    InvalidValue(#[from] BreacherDmgError),
}
