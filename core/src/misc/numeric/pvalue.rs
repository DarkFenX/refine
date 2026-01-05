use ordered_float::OrderedFloat;

use crate::{ad::AValue, misc::Value, util::LibDefault};

/// Positive float value.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct PValue(OrderedFloat<f64>);
impl PValue {
    pub const fn new_clamped(value: f64) -> Self {
        Self(OrderedFloat(value.max(0.0)))
    }
    pub(crate) const fn new_f64_unchecked(value: f64) -> Self {
        Self(OrderedFloat(value))
    }
    pub(crate) const fn new_of64_unchecked(value: OrderedFloat<f64>) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> f64 {
        self.0.into_inner()
    }
}
impl From<f64> for PValue {
    fn from(value: f64) -> Self {
        Self::new_clamped(value)
    }
}
impl From<PValue> for f64 {
    fn from(value: PValue) -> Self {
        value.0.into_inner()
    }
}
impl const LibDefault for PValue {
    fn lib_default() -> Self {
        Self(OrderedFloat(0.0))
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
        Self::new_clamped(value.into())
    }
}
