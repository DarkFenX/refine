use ordered_float::OrderedFloat;

use crate::{
    ad::AValue,
    misc::{Count, PValue},
    util::{FLOAT_TOLERANCE, LibMax, round, sig_round},
};

/// Float value.
#[derive(Copy, Clone, Default, Debug, derive_more::Display)]
pub struct Value(f64);
impl Value {
    pub(crate) const ZERO: Value = Value::new(0.0);
    pub(crate) const ONE: Value = Value::new(1.0);
    pub(crate) const HUNDRED: Value = Value::new(100.0);
    pub(crate) const FLOAT_TOLERANCE: Value = Value::new(FLOAT_TOLERANCE);

    pub const fn new(v: f64) -> Self {
        Self(v)
    }
    pub fn into_inner(self) -> f64 {
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
        value.0
    }
}
impl From<&Value> for f64 {
    fn from(value: &Value) -> Self {
        value.0
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
impl From<Count> for Value {
    fn from(value: Count) -> Self {
        Self::new(value.into_inner() as f64)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for Value {}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(self.0).eq(&OrderedFloat(other.0))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.0).cmp(&OrderedFloat(other.0))
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        OrderedFloat(self.0).partial_cmp(&OrderedFloat(other.0))
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat(self.0).hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Value {
    pub(crate) fn abs(self) -> Self {
        Self(self.0.abs())
    }
    pub(crate) fn min(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }
    pub(crate) fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    pub(crate) fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    pub(crate) fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    pub(crate) fn sig_rounded(self, digits: u32) -> Self {
        Self(sig_round(self.0, digits))
    }
    pub(crate) fn round_to_digits(&mut self, digits: i32) {
        self.0 = round(self.0, digits);
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
impl std::ops::SubAssign<Value> for Value {
    fn sub_assign(&mut self, rhs: Value) {
        self.0 -= rhs.0;
    }
}
// Multiplication
impl std::ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        Value(self.0 * rhs.0)
    }
}
impl std::ops::Mul<PValue> for Value {
    type Output = Value;
    fn mul(self, rhs: PValue) -> Self::Output {
        Value(self.0 * rhs.into_inner())
    }
}
impl std::ops::MulAssign<Value> for Value {
    fn mul_assign(&mut self, rhs: Value) {
        self.0 *= rhs.0;
    }
}
impl std::ops::MulAssign<PValue> for Value {
    fn mul_assign(&mut self, rhs: PValue) {
        self.0 *= rhs.into_inner();
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
impl std::ops::Div<PValue> for Value {
    type Output = Value;
    fn div(self, rhs: PValue) -> Self::Output {
        Value(self.0 / rhs.into_inner())
    }
}
impl std::ops::Div<&PValue> for Value {
    type Output = Value;
    fn div(self, rhs: &PValue) -> Self::Output {
        Value(self.0 / rhs.into_inner())
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
        Value(self.0.max(rhs))
    }
}
