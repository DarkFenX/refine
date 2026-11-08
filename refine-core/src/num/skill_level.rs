use std::{mem::transmute, pat::pattern_type, str::FromStr};

use crate::{Value, ad::ASkillLevel};

const MIN_RAW: u8 = 0;
const MAX_RAW: u8 = 5;
type Range = pattern_type!(u8 is MIN_RAW..=MAX_RAW);

/// Level a skill is trained to, within `[0, 5]` range.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[derive(Copy, Clone)]
pub struct SkillLevel(Range);
impl SkillLevel {
    pub fn from_i32_checked(level: i32) -> Result<Self, SkillLevelError> {
        match (Self::MIN.into_u8() as i32..=Self::MAX.into_u8() as i32).contains(&level) {
            true => Ok(Self::from_u8(level as u8)),
            false => Err(SkillLevelError { level }),
        }
    }
    pub const fn from_i32_clamped(level: i32) -> Self {
        Self::from_u8(level.clamp(Self::MIN.into_u8() as i32, Self::MAX.into_u8() as i32) as u8)
    }
    pub const fn from_u64_clamped(level: u64) -> Self {
        Self::from_u8(level.clamp(Self::MIN.into_u8() as u64, Self::MAX.into_u8() as u64) as u8)
    }
    pub const fn from_i64_clamped(level: i64) -> Self {
        Self::from_u8(level.clamp(Self::MIN.into_u8() as i64, Self::MAX.into_u8() as i64) as u8)
    }
    pub const fn into_u8(self) -> u8 {
        // SAFETY: any value in range can be represented by base type
        unsafe { transmute::<Range, u8>(self.0) }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "skill level {level} is out of allowed range [{}, {}]",
    SkillLevel::MIN,
    SkillLevel::MAX
)]
pub struct SkillLevelError {
    pub level: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillLevel {
    pub const DEFAULT: Self = Self::from_u8(MIN_RAW);
    pub const MIN: Self = Self::from_u8(MIN_RAW);
    pub const MAX: Self = Self::from_u8(MAX_RAW);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillLevel {
    const fn from_u8(level: u8) -> Self {
        assert!(level <= MAX_RAW);
        // SAFETY: just checked that the level is in range
        Self(unsafe { transmute::<u8, Range>(level) })
    }
    pub(crate) fn from_f64_rounded(level: f64) -> Self {
        Self::from_u8(
            level
                .clamp(Self::MIN.into_u8() as f64, Self::MAX.into_u8() as f64)
                .round() as u8,
        )
    }
    pub(crate) fn from_a_skill_level(a_skill_level: ASkillLevel) -> Self {
        Self::from_u8(a_skill_level.into_u8())
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64_unchecked(self.into_u8() as f64)
    }
}
impl From<SkillLevel> for u8 {
    fn from(v: SkillLevel) -> Self {
        v.into_u8()
    }
}
impl From<SkillLevel> for i32 {
    fn from(v: SkillLevel) -> Self {
        v.into_u8() as i32
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - pattern types do not provide those
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for SkillLevel {}
impl PartialEq for SkillLevel {
    fn eq(&self, other: &Self) -> bool {
        self.into_u8() == other.into_u8()
    }
}
impl std::marker::StructuralPartialEq for SkillLevel {}

impl Ord for SkillLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_u8().cmp(&other.into_u8())
    }
}
impl PartialOrd for SkillLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Pattern types have no Hash to derive
impl std::hash::Hash for SkillLevel {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.into_u8().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.into_u8(), f)
    }
}

impl FromStr for SkillLevel {
    type Err = SkillLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = i64::from_str(s) {
            return Ok(Self::from_i64_clamped(value));
        }
        let value = f64::from_str(s)?;
        if !value.is_finite() {
            return Err(SkillLevelParseError::NotFinite(value));
        }
        let value = Self::from_f64_rounded(value);
        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SkillLevelParseError {
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

    impl Serialize for SkillLevel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u8(self.into_u8())
        }
    }

    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = SkillLevel;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("number with skill level")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(SkillLevel::from_u64_clamped(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(SkillLevel::from_i64_clamped(v))
        }
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.is_finite() {
                true => Ok(SkillLevel::from_f64_rounded(v)),
                false => Err(Error::custom(SkillLevelParseError::NotFinite(v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for SkillLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(VisitorImpl)
        }
    }
}
