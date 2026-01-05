use ordered_float::OrderedFloat;

use crate::util::LibDefault;

const SS_MIN: f64 = -10.0;
const SS_MAX: f64 = 5.0;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct FitSecStatus(OrderedFloat<f64>);
impl FitSecStatus {
    pub fn new_checked(sec_status: f64) -> Result<Self, FitSecStatusError> {
        match (SS_MIN..=SS_MAX).contains(&sec_status) {
            true => Ok(Self(OrderedFloat(sec_status))),
            false => Err(FitSecStatusError { sec_status }),
        }
    }
    pub const fn new_clamped(sec_status: f64) -> Self {
        Self(OrderedFloat(sec_status.clamp(SS_MIN, SS_MAX)))
    }
    pub(crate) fn get_inner(&self) -> OrderedFloat<f64> {
        self.0
    }
}
impl From<FitSecStatus> for f64 {
    fn from(sec_status: FitSecStatus) -> Self {
        sec_status.0.into_inner()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("sec status {sec_status} is out of allowed range [-10, 5]")]
pub struct FitSecStatusError {
    pub sec_status: f64,
}
