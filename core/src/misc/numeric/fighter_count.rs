use crate::misc::{PValue, Value};

const COUNT_MIN: u32 = 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct FighterCount(u32);
impl FighterCount {
    pub fn from_u32_checked(count: u32) -> Result<Self, FighterCountError> {
        match (COUNT_MIN..).contains(&count) {
            true => Ok(Self(count)),
            false => Err(FighterCountError { count }),
        }
    }
    pub const fn from_u32_clamped(count: u32) -> Self {
        Self(count.max(COUNT_MIN))
    }
    pub const fn into_u32(self) -> u32 {
        self.0
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
    pub(crate) const ONE: Self = Self(1);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FighterCount {
    pub(crate) fn from_f64_rounded(count: f64) -> Self {
        Self(count.clamp(1.0, u32::MAX as f64).round() as u32)
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0 as f64)
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0 as f64)
    }
}
impl From<FighterCount> for u32 {
    fn from(v: FighterCount) -> Self {
        v.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Default for FighterCount {
    fn default() -> Self {
        Self(COUNT_MIN)
    }
}
