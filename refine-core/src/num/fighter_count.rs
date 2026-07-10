use std::num::NonZeroU32;

use crate::num::{PValue, Value};

const DEFAULT: NonZeroU32 = NonZeroU32::MIN;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct FighterCount(NonZeroU32);
impl FighterCount {
    pub fn from_u32_checked(count: u32) -> Result<Self, FighterCountError> {
        match NonZeroU32::try_from(count) {
            Ok(count) => Ok(Self(count)),
            Err(_) => Err(FighterCountError { count }),
        }
    }
    pub const fn from_u32_clamped(count: u32) -> Self {
        Self(NonZeroU32::try_from(count).unwrap_or(DEFAULT))
    }
    pub const fn into_u32(self) -> u32 {
        self.0.get()
    }
}
#[derive(thiserror::Error, Debug)]
#[error("fighter count should be 1+, received {count}")]
pub struct FighterCountError {
    pub count: u32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterCount {
    pub(crate) const ONE: Self = Self(NonZeroU32::MIN);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterCount {
    pub(crate) fn from_f64_rounded(count: f64) -> Self {
        Self(
            NonZeroU32::try_from(
                count
                    .clamp(NonZeroU32::MIN.get() as f64, NonZeroU32::MAX.get() as f64)
                    .round() as u32,
            )
            .unwrap(),
        )
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0.get() as f64)
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0.get() as f64)
    }
}
impl From<FighterCount> for u32 {
    fn from(v: FighterCount) -> Self {
        v.0.get()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Default for FighterCount {
    fn default() -> Self {
        Self(DEFAULT)
    }
}
