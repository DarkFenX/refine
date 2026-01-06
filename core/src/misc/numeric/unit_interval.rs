use ordered_float::OrderedFloat;

use crate::misc::Value;

#[derive(Copy, Clone, Debug, derive_more::Display)]
pub struct UnitInterval(f64);
impl UnitInterval {
    pub fn new_checked(v: f64) -> Result<Self, UnitIntervalError> {
        match (0.0..=1.0).contains(&v) {
            true => Ok(Self(v)),
            false => Err(UnitIntervalError { value: v }),
        }
    }
    pub fn new_clamped(v: f64) -> Self {
        Self(v.clamp(0.0, 1.0))
    }
    pub fn into_inner(self) -> f64 {
        self.0
    }
}

#[derive(thiserror::Error, Debug)]
#[error("value {value} is out of allowed unit interval range [0, 1]")]
pub struct UnitIntervalError {
    pub value: f64,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Mul<Value> for UnitInterval {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Value::new(self.0 * rhs.into_inner())
    }
}
impl std::ops::Mul<UnitInterval> for Value {
    type Output = Value;

    fn mul(self, rhs: UnitInterval) -> Self::Output {
        Value::new(self.into_inner() * rhs.0)
    }
}
