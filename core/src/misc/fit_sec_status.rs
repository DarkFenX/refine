use crate::def::OF;

const SS_MIN: f64 = -10.0;
const SS_MAX: f64 = 5.0;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FitSecStatus {
    inner: OF<f64>,
}
impl FitSecStatus {
    pub fn new_checked(sec_status: impl Into<f64>) -> Result<Self, FitSecStatusError> {
        let sec_status: f64 = sec_status.into();
        match (SS_MIN..=SS_MAX).contains(&sec_status) {
            true => Ok(Self { inner: OF(sec_status) }),
            false => Err(FitSecStatusError { sec_status }),
        }
    }
    pub fn new_clamped(sec_status: impl Into<f64>) -> Self {
        Self {
            inner: OF(f64::clamp(sec_status.into(), SS_MIN, SS_MAX)),
        }
    }
    pub fn get_inner(&self) -> OF<f64> {
        self.inner
    }
}
impl std::fmt::Display for FitSecStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("sec status {sec_status} is out of allowed range [-10, 5]")]
pub struct FitSecStatusError {
    pub sec_status: f64,
}
