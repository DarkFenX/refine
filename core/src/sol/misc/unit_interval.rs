use ordered_float::OrderedFloat as OF;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnitInterval {
    inner: OF<f64>,
}
impl UnitInterval {
    pub fn new_checked(value: impl Into<f64>) -> Result<Self, UnitIntervalError> {
        let value: f64 = value.into();
        match value >= 0.0 && value <= 1.0 {
            true => Ok(Self { inner: OF(value) }),
            false => Err(UnitIntervalError { value }),
        }
    }
    pub fn new_clamped(value: impl Into<f64>) -> Self {
        Self::new_clamped_of64(OF(value.into()))
    }
    pub(in crate::sol) fn new_clamped_of64(value: OF<f64>) -> Self {
        Self {
            inner: OF(0.0).max(OF(1.0).min(value)),
        }
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
