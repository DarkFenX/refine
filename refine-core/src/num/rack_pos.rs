use std::{mem::transmute, pat::pattern_type, str::FromStr};

const MIN_RAW: u16 = 0;
const MAX_RAW: u16 = 999;
type Range = pattern_type!(u16 is MIN_RAW..=MAX_RAW);

/// Zero-based position of a module within its rack, within `[0, 999]` range.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[derive(Copy, Clone)]
pub struct RackPos(Range);
impl RackPos {
    pub fn from_usize_checked(pos: usize) -> Result<Self, RackPosError> {
        match pos <= Self::MAX.into_usize() {
            true => Ok(Self::from_u16(pos as u16)),
            false => Err(RackPosError { pos }),
        }
    }
    pub const fn from_usize_clamped(pos: usize) -> Self {
        Self::from_u16(pos.clamp(Self::MIN.into_usize(), Self::MAX.into_usize()) as u16)
    }
    pub const fn from_u64_clamped(pos: u64) -> Self {
        Self::from_u16(pos.clamp(Self::MIN.into_usize() as u64, Self::MAX.into_usize() as u64) as u16)
    }
    pub const fn from_i64_clamped(pos: i64) -> Self {
        Self::from_u16(pos.clamp(Self::MIN.into_usize() as i64, Self::MAX.into_usize() as i64) as u16)
    }
    pub const fn into_usize(self) -> usize {
        self.into_u16() as usize
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
    pub const MIN: Self = Self::from_u16(MIN_RAW);
    pub const MAX: Self = Self::from_u16(MAX_RAW);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RackPos {
    const fn from_u16(pos: u16) -> Self {
        assert!(pos <= MAX_RAW);
        // SAFETY: just checked that the position is in range
        Self(unsafe { transmute::<u16, Range>(pos) })
    }
    const fn into_u16(self) -> u16 {
        // SAFETY: any value in range can be represented by base type
        unsafe { transmute::<Range, u16>(self.0) }
    }
    pub(crate) fn from_f64_rounded(pos: f64) -> Self {
        Self::from_u16(
            pos.clamp(Self::MIN.into_usize() as f64, Self::MAX.into_usize() as f64)
                .round() as u16,
        )
    }
}
impl From<RackPos> for usize {
    fn from(v: RackPos) -> Self {
        v.into_usize()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RackPos {
    // Callers ensure the result stays within range
    pub(crate) fn inc_unchecked(self) -> Self {
        Self::from_u16(self.into_u16() + 1)
    }
    pub(crate) fn dec_unchecked(self) -> Self {
        Self::from_u16(self.into_u16() - 1)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - pattern types do not provide those
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for RackPos {}
impl PartialEq for RackPos {
    fn eq(&self, other: &Self) -> bool {
        self.into_u16() == other.into_u16()
    }
}
impl std::marker::StructuralPartialEq for RackPos {}

impl Ord for RackPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_u16().cmp(&other.into_u16())
    }
}
impl PartialOrd for RackPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Display for RackPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.into_usize(), f)
    }
}

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
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for RackPos {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u16(self.into_u16())
        }
    }

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
            deserializer.deserialize_any(VisitorImpl)
        }
    }
}
