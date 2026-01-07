use crate::{
    err::basic::BreacherDmgError,
    misc::{PValue, UnitInterval, Value},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Breacher {
    absolute_max: PValue,
    relative_max: UnitInterval,
}
impl Breacher {
    pub fn new_clamped(absolute_max: Value, relative_max: Value) -> Self {
        Self {
            absolute_max: PValue::from_value_clamped(absolute_max),
            relative_max: UnitInterval::from_value_clamped(relative_max),
        }
    }
    pub fn try_new(absolute_max: Value, relative_max: Value) -> Result<Self, BreacherError> {
        let absolute_max = match absolute_max >= Value::ZERO {
            true => PValue::from_val_unchecked(absolute_max),
            false => return Err(BreacherDmgError::Absolute(absolute_max).into()),
        };
        let relative_max = match UnitInterval::from_f64_checked(relative_max.into_f64()) {
            Ok(relative_max) => relative_max,
            Err(_) => return Err(BreacherDmgError::Relative(relative_max).into()),
        };
        Ok(Self {
            absolute_max,
            relative_max,
        })
    }
    pub fn get_absolute_max(&self) -> PValue {
        self.absolute_max
    }
    pub fn get_relative_max(&self) -> UnitInterval {
        self.relative_max
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BreacherError {
    #[error("{0}")]
    InvalidValue(#[from] BreacherDmgError),
}
