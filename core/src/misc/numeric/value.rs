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
    pub const fn from_f64(v: f64) -> Self {
        Self(v)
    }
    pub const fn into_f64(self) -> f64 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Value {
    pub(crate) const ZERO: Self = Self::from_f64(0.0);
    pub(crate) const ONE: Self = Self::from_f64(1.0);
    pub(crate) const TWO: Self = Self::from_f64(2.0);
    pub(crate) const HUNDRED: Self = Self::from_f64(100.0);
    pub(crate) const THOUSAND: Self = Self::from_f64(1000.0);
    pub(crate) const HUNDREDTH: Self = Self::from_f64(0.01);
    pub(crate) const FLOAT_TOLERANCE: Self = Self::from_f64(FLOAT_TOLERANCE);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Value {
    pub(crate) fn from_a_value(a_value: AValue) -> Self {
        Self(a_value.into_f64())
    }
    pub(crate) fn from_pvalue(pvalue: PValue) -> Self {
        Self(pvalue.into_f64())
    }
    pub(crate) fn from_count(count: Count) -> Self {
        Self(count.into_u32() as f64)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for Value {}
impl std::marker::StructuralPartialEq for Value {}
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
    pub(crate) fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
    pub(crate) fn abs(self) -> PValue {
        PValue::from_f64_unchecked(self.0.abs())
    }
    pub(crate) fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    pub(crate) fn exp(self) -> PValue {
        PValue::from_f64_unchecked(self.0.exp())
    }
    pub(crate) fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    pub(crate) fn pow2(self) -> PValue {
        PValue::from_f64_unchecked(self.0.powi(2))
    }
    pub(crate) fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    pub(crate) fn sig_rounded(self, digits: u32) -> Self {
        Self(sig_round(self.0, digits))
    }
    pub(crate) fn rounded_to_digits(&mut self, digits: i32) -> Self {
        Self(round(self.0, digits))
    }
    pub(crate) fn round_to_digits(&mut self, digits: i32) {
        self.0 = round(self.0, digits);
    }
    pub(crate) fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }
}
impl std::ops::Neg for Value {
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
impl std::ops::AddAssign<PValue> for Value {
    fn add_assign(&mut self, rhs: PValue) {
        self.0 += rhs.into_f64();
    }
}
// Subtraction
impl std::ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Self::Output {
        Value(self.0 - rhs.0)
    }
}
impl std::ops::Sub<PValue> for Value {
    type Output = Value;
    fn sub(self, rhs: PValue) -> Self::Output {
        Value(self.0 - rhs.into_f64())
    }
}
impl std::ops::SubAssign<Value> for Value {
    fn sub_assign(&mut self, rhs: Value) {
        self.0 -= rhs.0;
    }
}
impl std::ops::SubAssign<PValue> for Value {
    fn sub_assign(&mut self, rhs: PValue) {
        self.0 -= rhs.into_f64();
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
        Value(self.0 * rhs.into_f64())
    }
}
impl std::ops::MulAssign<Value> for Value {
    fn mul_assign(&mut self, rhs: Value) {
        self.0 *= rhs.0;
    }
}
impl std::ops::MulAssign<PValue> for Value {
    fn mul_assign(&mut self, rhs: PValue) {
        self.0 *= rhs.into_f64();
    }
}
// Division
impl std::ops::Div<f64> for Value {
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
impl std::ops::Div<PValue> for Value {
    type Output = Value;
    fn div(self, rhs: PValue) -> Self::Output {
        Value(self.0 / rhs.into_f64())
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
