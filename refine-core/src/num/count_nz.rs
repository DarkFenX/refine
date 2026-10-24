use std::{num::NonZeroU32, str::FromStr};

use crate::{
    num::{PValue, Value},
    util::round_f64_to_u32,
};

const DEFAULT: NonZeroU32 = NonZeroU32::MIN;

/// Count of something which cannot be zero, e.g. of fighters in a squad.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct CountNz(NonZeroU32);
impl CountNz {
    pub fn from_u32_checked(count: u32) -> Result<Self, CountNzError> {
        match NonZeroU32::try_from(count) {
            Ok(count) => Ok(Self(count)),
            Err(..) => Err(CountNzError),
        }
    }
    pub const fn from_u32_clamped(count: u32) -> Self {
        Self(NonZeroU32::try_from(count).unwrap_or(DEFAULT))
    }
    pub const fn from_u64_clamped(count: u64) -> Self {
        Self::from_u32_clamped(count.saturating_cast())
    }
    pub const fn from_i64_clamped(count: i64) -> Self {
        Self::from_u32_clamped(count.saturating_cast())
    }
    pub const fn into_u32(self) -> u32 {
        self.0.get()
    }
}
#[derive(Debug, thiserror::Error)]
#[error("non-zero count should be 1+, received 0")]
pub struct CountNzError;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CountNz {
    pub(crate) const ONE: Self = Self(NonZeroU32::MIN);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CountNz {
    pub(crate) fn from_f64_rounded(count: f64) -> Self {
        Self::from_u32_clamped(round_f64_to_u32(count))
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0.get() as f64)
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0.get() as f64)
    }
}
impl From<CountNz> for u32 {
    fn from(v: CountNz) -> Self {
        v.0.get()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
const impl Default for CountNz {
    fn default() -> Self {
        Self(DEFAULT)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for CountNz {
    type Err = CountNzParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = i64::from_str(s) {
            return Ok(Self::from_i64_clamped(value));
        }
        let value = f64::from_str(s)?;
        if !value.is_finite() {
            return Err(CountNzParseError::NotFinite(value));
        }
        let value = Self::from_f64_rounded(value);
        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CountNzParseError {
    #[error(transparent)]
    InvalidNumber(#[from] std::num::ParseFloatError),
    #[error("value {0} is not finite")]
    NotFinite(f64),
}

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = CountNz;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("number with non-zero count")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(CountNz::from_u64_clamped(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(CountNz::from_i64_clamped(v))
        }
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.is_finite() {
                true => Ok(CountNz::from_f64_rounded(v)),
                false => Err(Error::custom(CountNzParseError::NotFinite(v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for CountNz {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u32(VisitorImpl)
        }
    }
}
