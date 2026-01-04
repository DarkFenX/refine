use ordered_float::OrderedFloat;

use crate::{ad::AValue, util::Max};

/// Float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Value(OrderedFloat<f64>);
impl Value {
    pub fn new(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.0.into_inner()
    }
}
impl From<&Value> for f64 {
    fn from(value: &Value) -> Self {
        value.0.into_inner()
    }
}
// Conversions between lib-specific types
impl From<AValue> for Value {
    fn from(value: AValue) -> Self {
        Self::new(value.into_inner())
    }
}
impl From<&AValue> for Value {
    fn from(value: &AValue) -> Self {
        Self::new(value.into_inner())
    }
}
// Math operations
impl std::ops::Div<f64> for Value {
    type Output = Value;

    fn div(self, rhs: f64) -> Self::Output {
        Value(self.0 / rhs)
    }
}
impl std::ops::Div<f64> for &Value {
    type Output = Value;

    fn div(self, rhs: f64) -> Self::Output {
        Value(self.0 / rhs)
    }
}
impl Max<f64> for Value {
    fn max(self, rhs: f64) -> Self {
        Value(self.0.max(OrderedFloat(rhs)))
    }
}
