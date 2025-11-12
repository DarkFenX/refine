use crate::def::{AttrVal, OF};

const VAL_MIN: AttrVal = OF(0.0);
const VAL_MAX: AttrVal = OF(1.0);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitInterval {
    inner: AttrVal,
}
impl UnitInterval {
    pub fn new_checked(value: impl Into<f64>) -> Result<Self, UnitIntervalError> {
        let value = OF(value.into());
        match (VAL_MIN..=VAL_MAX).contains(&value) {
            true => Ok(Self { inner: value }),
            false => Err(UnitIntervalError {
                value: value.into_inner(),
            }),
        }
    }
    pub fn new_clamped(value: impl Into<f64>) -> Self {
        Self::new_clamped_of64(OF(value.into()))
    }
    pub(crate) fn new_clamped_of64(value: OF<f64>) -> Self {
        Self {
            inner: value.clamp(VAL_MIN, VAL_MAX),
        }
    }
    pub(crate) const fn new_const(value: OF<f64>) -> Self {
        Self { inner: value }
    }
    pub fn get_inner(&self) -> OF<f64> {
        self.inner
    }
}
impl std::fmt::Display for UnitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("value {value} is out of allowed unit interval range [0, 1]")]
pub struct UnitIntervalError {
    pub value: f64,
}
