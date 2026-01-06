use ordered_float::OrderedFloat;

use crate::{
    ad::AValue,
    misc::Value,
    util::{LibDefault, sig_round},
};

/// Positive float value.
#[derive(Copy, Clone, Default, Debug, derive_more::Display)]
pub struct PValue(f64);
impl PValue {
    pub(crate) const ZERO: PValue = PValue::new_unchecked(0.0);
    pub(crate) const ONE: PValue = PValue::new_unchecked(1.0);

    pub const fn new_clamped(v: f64) -> Self {
        Self(v.max(0.0))
    }
    pub(crate) const fn new_unchecked(v: f64) -> Self {
        Self(v)
    }
    pub fn into_inner(self) -> f64 {
        self.0
    }
}
impl From<f64> for PValue {
    fn from(value: f64) -> Self {
        Self::new_clamped(value)
    }
}
impl From<PValue> for f64 {
    fn from(value: PValue) -> Self {
        value.0
    }
}
impl const LibDefault for PValue {
    fn lib_default() -> Self {
        Self(0.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions between lib-specific types
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<AValue> for PValue {
    fn from(value: AValue) -> Self {
        Self::new_clamped(value.into_inner())
    }
}
impl From<&AValue> for PValue {
    fn from(value: &AValue) -> Self {
        Self::new_clamped(value.into_inner())
    }
}
impl From<Value> for PValue {
    fn from(value: Value) -> Self {
        Self::new_clamped(value.into_inner())
    }
}
impl From<PValue> for Value {
    fn from(value: PValue) -> Self {
        Self::new(value.into_inner())
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
        PValue::new_clamped(self.0 - rhs.0)
    }
}
// Division
impl std::ops::Div<&PValue> for PValue {
    type Output = PValue;
    fn div(self, rhs: &PValue) -> Self::Output {
        PValue(self.0 / rhs.0)
    }
}
