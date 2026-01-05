use ordered_float::{Float, OrderedFloat};

use crate::{
    ad::AValue,
    util::{FLOAT_TOLERANCE, LibMax},
};

/// Float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Value(OrderedFloat<f64>);
impl Value {
    pub(crate) const ZERO: Value = Value::new(0.0);
    pub(crate) const ONE: Value = Value::new(1.0);
    pub(crate) const HUNDRED: Value = Value::new(100.0);
    pub(crate) const FLOAT_TOLERANCE: Value = Value::new(FLOAT_TOLERANCE);

    pub const fn new(v: f64) -> Self {
        Self(OrderedFloat(v))
    }
    pub fn into_inner(self) -> f64 {
        self.0.into_inner()
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
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    pub fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}
impl std::ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}
impl std::ops::Neg for &Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}
// Addition
impl std::ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign<Value> for Value {
    fn add_assign(&mut self, rhs: Value) {
        self.0 += rhs.0;
    }
}
// Subtraction
impl std::ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Self::Output {
        Value(self.0 - rhs.0)
    }
}
impl std::ops::Sub<Value> for &Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Self::Output {
        Value(self.0 - rhs.0)
    }
}
// Multiplication
impl std::ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        Value(self.0 * rhs.0)
    }
}
impl std::ops::MulAssign<Value> for Value {
    fn mul_assign(&mut self, rhs: Value) {
        self.0 *= rhs.0;
    }
}
// Division
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
impl std::ops::Div<&Value> for Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Self::Output {
        Value(self.0 / rhs.0)
    }
}
// Sum
impl std::iter::Sum<Value> for Value {
    fn sum<I: Iterator<Item = Value>>(iter: I) -> Self {
        Value(iter.map(|v| v.0).sum())
    }
}
impl<'a> std::iter::Sum<&'a Value> for Value {
    fn sum<I: Iterator<Item = &'a Value>>(iter: I) -> Self {
        Value(iter.map(|v| v.0).sum())
    }
}
// Product
impl std::iter::Product<Value> for Value {
    fn product<I: Iterator<Item = Value>>(iter: I) -> Self {
        Value(iter.map(|v| v.0).product())
    }
}
impl<'a> std::iter::Product<&'a Value> for Value {
    fn product<I: Iterator<Item = &'a Value>>(iter: I) -> Self {
        Value(iter.map(|v| v.0).product())
    }
}
// Others
impl LibMax<f64> for Value {
    fn lib_max(self, rhs: f64) -> Self {
        Value(Float::max(self.0, OrderedFloat(rhs)))
    }
}
