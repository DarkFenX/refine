use crate::def::Count;

const COUNT_MIN: Count = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FighterCountOverride {
    inner: Count,
}
impl FighterCountOverride {
    pub fn new_checked(count: impl Into<Count>) -> Result<Self, FighterCountOverrideError> {
        let count = count.into();
        match (COUNT_MIN..).contains(&count) {
            true => Ok(Self { inner: count }),
            false => Err(FighterCountOverrideError { count }),
        }
    }
    pub fn new_clamped(count: impl Into<Count>) -> Self {
        Self {
            inner: COUNT_MIN.max(count.into()),
        }
    }
    pub fn get_inner(&self) -> Count {
        self.inner
    }
}
impl std::fmt::Display for FighterCountOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("fighter count should be 1+, received {count}")]
pub struct FighterCountOverrideError {
    pub count: Count,
}
