use ordered_float::OrderedFloat;

use crate::{
    ad::AValue,
    def::SERVER_TICK_HZ,
    misc::Value,
    util::{FLOAT_TOLERANCE, ceil_tick, ceil_unerr, floor_tick, floor_unerr, sig_round},
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
    pub(crate) const TEN: Self = Self::from_f64_clamped(10.0);
    pub(crate) const FLOAT_TOLERANCE: Self = Self::from_f64_clamped(FLOAT_TOLERANCE);
    pub(crate) const SERVER_TICK_HZ: Self = Self::from_f64_clamped(SERVER_TICK_HZ as f64);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PValue {
    pub(crate) fn from_a_value_clamped(value: AValue) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn from_value_clamped(value: Value) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn from_value_unchecked(value: Value) -> Self {
        Self::from_f64_unchecked(value.into_f64())
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for PValue {}
impl std::marker::StructuralPartialEq for PValue {}
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
    pub(crate) fn max_value(self, other: Value) -> Self {
        Self(self.0.max(other.into_f64()))
    }
    pub(crate) fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    pub(crate) fn pow_pvalue(self, n: Self) -> Self {
        Self(self.0.powf(n.into_f64()))
    }
    pub(crate) fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }
    pub(crate) fn is_nan(self) -> bool {
        self.0.is_nan()
    }
    pub(crate) fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    pub(crate) fn sig_rounded(self, digits: u32) -> Self {
        Self(sig_round(self.0, digits))
    }
    pub(crate) fn fract(self) -> Self {
        Self(self.0.fract())
    }
    pub(crate) fn floor_unerr(self) -> Self {
        Self(floor_unerr(self.0))
    }
    pub(crate) fn ceil_unerr(self) -> Self {
        Self(ceil_unerr(self.0))
    }
    // Tick-specific math
    pub(crate) fn floor_tick(self) -> PValue {
        PValue(floor_tick(self.0))
    }
    pub(crate) fn ceil_tick(self) -> PValue {
        PValue(ceil_tick(self.0))
    }
}
impl std::ops::Neg for PValue {
    type Output = Value;
    fn neg(self) -> Self::Output {
        Value::from_f64(-self.0)
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
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        Value::from_f64(self.0 - rhs.0)
    }
}
// Multiplication
impl std::ops::Mul<PValue> for PValue {
    type Output = PValue;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl std::ops::Mul<Value> for PValue {
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        Value::from_f64(self.0 * rhs.into_f64())
    }
}
impl std::ops::MulAssign<PValue> for PValue {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
// Division
impl std::ops::Div<PValue> for PValue {
    type Output = PValue;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl std::ops::Div<Value> for PValue {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        Value::from_f64(self.0 / rhs.into_f64())
    }
}
