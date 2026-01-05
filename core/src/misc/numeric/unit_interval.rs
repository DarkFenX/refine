use ordered_float::OrderedFloat;

use crate::misc::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct UnitInterval(OrderedFloat<f64>);
impl UnitInterval {
    pub fn new_checked(value: f64) -> Result<Self, UnitIntervalError> {
        match (0.0..=1.0).contains(&value) {
            true => Ok(Self(OrderedFloat(value))),
            false => Err(UnitIntervalError { value }),
        }
    }
    pub fn new_clamped(value: f64) -> Self {
        Self(OrderedFloat(value.clamp(0.0, 1.0)))
    }
    pub fn into_inner(self) -> f64 {
        self.0.into_inner()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("value {value} is out of allowed unit interval range [0, 1]")]
pub struct UnitIntervalError {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Mul<Value> for UnitInterval {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Value::new_of64(self.0 * rhs.into_inner_of64())
    }
}
impl std::ops::Mul<UnitInterval> for Value {
    type Output = Value;

    fn mul(self, rhs: UnitInterval) -> Self::Output {
        Value::new_of64(self.into_inner_of64() * rhs.0)
    }
}
