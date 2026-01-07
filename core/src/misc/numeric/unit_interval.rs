use ordered_float::OrderedFloat;

use crate::{
    misc::{PValue, Value},
    util::FLOAT_TOLERANCE,
};

#[derive(Copy, Clone, Debug, derive_more::Display)]
pub struct UnitInterval(f64);
impl UnitInterval {
    pub fn from_f64_checked(v: f64) -> Result<Self, UnitIntervalError> {
        match (0.0..=1.0).contains(&v) {
            true => Ok(Self(v)),
            false => Err(UnitIntervalError { value: v }),
        }
    }
    pub const fn from_f64_clamped(v: f64) -> Self {
        Self(v.clamp(0.0, 1.0))
    }
    pub const fn into_f64(self) -> f64 {
        self.0
    }
}

#[derive(thiserror::Error, Debug)]
#[error("value {value} is out of allowed unit interval range [0, 1]")]
pub struct UnitIntervalError {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UnitInterval {
    pub(crate) const ZERO: Self = Self::from_f64_clamped(0.0);
    pub(crate) const ONE: Self = Self::from_f64_clamped(1.0);
    pub(crate) const FLOAT_TOLERANCE: Self = Self::from_f64_clamped(FLOAT_TOLERANCE);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UnitInterval {
    pub(crate) fn from_value_clamped(value: Value) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn from_pvalue_clamped(value: PValue) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0)
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for UnitInterval {}
impl PartialEq for UnitInterval {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(self.0).eq(&OrderedFloat(other.0))
    }
}

impl Ord for UnitInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.0).cmp(&OrderedFloat(other.0))
    }
}
impl PartialOrd for UnitInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        OrderedFloat(self.0).partial_cmp(&OrderedFloat(other.0))
    }
}

impl std::hash::Hash for UnitInterval {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat(self.0).hash(state);
    }
}
