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
    pub fn new_clamped(absolute_max: impl Into<f64>, relative_max: impl Into<f64>) -> Self {
        Self {
            absolute_max: OF(absolute_max.into()).max(ABS_MIN),
            relative_max: OF(relative_max.into()).clamp(REL_MIN, REL_MAX),
        }
    }
    pub fn try_new(absolute_max: impl Into<f64>, relative_max: impl Into<f64>) -> Result<Self, BreacherInfoError> {
        let absolute_max = OF(absolute_max.into());
        let relative_max = OF(relative_max.into());
        if absolute_max < ABS_MIN {
            return Err(BreacherDmgError::Absolute(absolute_max).into());
        }
        match (REL_MIN..=REL_MAX).contains(&relative_max) {
            true => Ok(Self {
                absolute_max,
                relative_max,
            }),
            false => Err(BreacherDmgError::Absolute(relative_max).into()),
        }
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
