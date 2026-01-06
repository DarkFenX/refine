use ordered_float::OrderedFloat;

use crate::{
    ad::AValue,
    misc::Value,
    util::{FLOAT_TOLERANCE, sig_round},
};

/// Positive float value.
#[derive(Copy, Clone, Default, Debug, derive_more::Display)]
pub struct PValue(f64);
impl PValue {
    pub const fn from_f64_clamped(v: f64) -> Self {
        Self(v.max(0.0))
    }
    pub(crate) const fn from_f64_unchecked(v: f64) -> Self {
        Self(v)
    }
    pub fn into_f64(self) -> f64 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PValue {
    pub(crate) const ZERO: Self = Self::from_f64_clamped(0.0);
    pub(crate) const ONE: Self = Self::from_f64_clamped(1.0);
    pub(crate) const FLOAT_TOLERANCE: Self = Self::from_f64_clamped(FLOAT_TOLERANCE);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PValue {
    pub(crate) fn from_a_val_clamped(value: AValue) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn from_val_clamped(value: Value) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for PValue {}
impl PartialEq for PValue {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(self.0).eq(&OrderedFloat(other.0))
    }
}

impl Ord for PValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.0).cmp(&OrderedFloat(other.0))
    }
}
impl PartialOrd for PValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        OrderedFloat(self.0).partial_cmp(&OrderedFloat(other.0))
    }
}

impl std::hash::Hash for PValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat(self.0).hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PValue {
    pub(crate) fn sig_rounded(self, digits: u32) -> Self {
        Self(sig_round(self.0, digits))
    }
}
// Addition
impl std::ops::Add<PValue> for PValue {
    type Output = PValue;
    fn add(self, rhs: PValue) -> Self::Output {
        PValue(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign<PValue> for PValue {
    fn add_assign(&mut self, rhs: PValue) {
        self.0 += rhs.0;
    }
}
// Subtraction
impl std::ops::Sub<PValue> for PValue {
    type Output = PValue;
    fn sub(self, rhs: PValue) -> Self::Output {
        PValue::from_f64_clamped(self.0 - rhs.0)
    }
}
// Multiplication
impl std::ops::Mul<PValue> for PValue {
    type Output = PValue;
    fn mul(self, rhs: PValue) -> Self::Output {
        PValue(self.0 * rhs.0)
    }
}
impl std::ops::MulAssign<PValue> for PValue {
    fn mul_assign(&mut self, rhs: PValue) {
        self.0 *= rhs.0;
    }
}
// Division
impl std::ops::Div<&PValue> for PValue {
    type Output = PValue;
    fn div(self, rhs: &PValue) -> Self::Output {
        PValue(self.0 / rhs.0)
    }
}
impl std::ops::Div<Value> for PValue {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        Value::from_f64(self.0 / rhs.into_f64())
    }
}
