use std::str::FromStr;

/// Zero-based position of a module within its rack, within `[0, 999]` range.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct RackPos(u16);
impl RackPos {
    pub fn from_usize_checked(pos: usize) -> Result<Self, RackPosError> {
        match pos <= Self::MAX.0 as usize {
            true => Ok(Self(pos as u16)),
            false => Err(RackPosError { pos }),
        }
    }
    pub const fn from_usize_clamped(pos: usize) -> Self {
        Self(pos.clamp(Self::MIN.0 as usize, Self::MAX.0 as usize) as u16)
    }
    pub const fn from_u64_clamped(pos: u64) -> Self {
        Self(pos.clamp(Self::MIN.0 as u64, Self::MAX.0 as u64) as u16)
    }
    pub const fn from_i64_clamped(pos: i64) -> Self {
        Self(pos.clamp(Self::MIN.0 as i64, Self::MAX.0 as i64) as u16)
    }
    pub const fn into_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, thiserror::Error)]
#[error("rack position {pos} is out of allowed range [{}, {}]", RackPos::MIN, RackPos::MAX)]
pub struct RackPosError {
    pub pos: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RackPos {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(999);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RackPos {
    pub(crate) fn from_f64_rounded(pos: f64) -> Self {
        Self(pos.clamp(Self::MIN.0 as f64, Self::MAX.0 as f64).round() as u16)
    }
}
impl From<RackPos> for usize {
    fn from(v: RackPos) -> Self {
        v.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RackPos {
    // Callers ensure the result stays within range
    pub(crate) fn inc_unchecked(self) -> Self {
        debug_assert!(self < Self::MAX);
        Self(self.0 + 1)
    }
    pub(crate) fn dec_unchecked(self) -> Self {
        Self(self.0 - 1)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for RackPos {
    type Err = RackPosParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = i64::from_str(s) {
            return Ok(Self::from_i64_clamped(value));
        }
        let value = f64::from_str(s)?;
        if !value.is_finite() {
            return Err(RackPosParseError::NotFinite(value));
        }
        let value = Self::from_f64_rounded(value);
        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RackPosParseError {
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
        type Value = RackPos;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("number with rack position")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(RackPos::from_u64_clamped(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(RackPos::from_i64_clamped(v))
        }
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.is_finite() {
                true => Ok(RackPos::from_f64_rounded(v)),
                false => Err(Error::custom(RackPosParseError::NotFinite(v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for RackPos {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u16(VisitorImpl)
        }
    }
}
