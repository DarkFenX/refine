use crate::def::OF;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FitSecStatus {
    inner: OF<f64>,
}
impl FitSecStatus {
    pub fn new_checked(sec_status: impl Into<f64>) -> Result<Self, FitSecStatusError> {
        let sec_status: f64 = sec_status.into();
        match (-10.0..=5.0).contains(&sec_status) {
            true => Ok(Self { inner: OF(sec_status) }),
            false => Err(FitSecStatusError { sec_status }),
        }
    }
    pub fn new_clamped(sec_status: impl Into<f64>) -> Self {
        Self {
            inner: OF(-10.0).max(OF(5.0).min(OF(sec_status.into()))),
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
