use ordered_float::{Float, OrderedFloat};

use crate::{ad::AValue, util::LibMax};

/// Float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Value(OrderedFloat<f64>);
impl Value {
    pub const fn new(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
    pub(super) fn new_of64(value: OrderedFloat<f64>) -> Self {
        Self(value)
    }
    pub(crate) fn into_inner_of64(self) -> OrderedFloat<f64> {
        self.0
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions between lib-specific types
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Value {
    pub fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}
impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl std::ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
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
impl std::ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Value(self.0 / rhs.0)
    }
}
impl LibMax<f64> for Value {
    fn lib_max(self, rhs: f64) -> Self {
        Value(Float::max(self.0, OrderedFloat(rhs)))
    }
}
