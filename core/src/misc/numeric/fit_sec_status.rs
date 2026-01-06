use ordered_float::OrderedFloat;

use crate::misc::Value;

const SS_MIN: f64 = -10.0;
const SS_MAX: f64 = 5.0;

#[derive(Copy, Clone, Default, Debug, derive_more::Display)]
pub struct FitSecStatus(f64);
impl FitSecStatus {
    pub fn from_f64_checked(sec_status: f64) -> Result<Self, FitSecStatusError> {
        match (SS_MIN..=SS_MAX).contains(&sec_status) {
            true => Ok(Self(sec_status)),
            false => Err(FitSecStatusError { sec_status }),
        }
    }
    pub const fn from_f64_clamped(sec_status: f64) -> Self {
        Self(sec_status.clamp(SS_MIN, SS_MAX))
    }
    pub const fn into_f64(self) -> f64 {
        self.0
    }
}
#[derive(thiserror::Error, Debug)]
#[error("sec status {sec_status} is out of allowed range [-10, 5]")]
pub struct FitSecStatusError {
    pub sec_status: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitSecStatus {
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality - those are implemented manually using conversion of values into ordered
// floats. It'd be much easier to put ordered float into struct instead, but it screws ability to
// use constants in pattern matching arms.
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for FitSecStatus {}
impl PartialEq for FitSecStatus {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(self.0).eq(&OrderedFloat(other.0))
    }
}

impl Ord for FitSecStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.0).cmp(&OrderedFloat(other.0))
    }
}
impl PartialOrd for FitSecStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        OrderedFloat(self.0).partial_cmp(&OrderedFloat(other.0))
    }
}
