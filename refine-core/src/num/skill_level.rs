use crate::{ad::ASkillLevel, num::Value};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, derive_more::Display)]
pub struct SkillLevel(u8);
impl SkillLevel {
    pub fn from_i32_checked(level: i32) -> Result<Self, SkillLevelError> {
        match (0..=5).contains(&level) {
            true => Ok(Self(level as u8)),
            false => Err(SkillLevelError { level }),
        }
    }
    pub const fn from_i32_clamped(level: i32) -> Self {
        Self(level.clamp(0, 5) as u8)
    }
    pub const fn into_u8(self) -> u8 {
        self.0
    }
}
#[derive(thiserror::Error, Debug)]
#[error("skill level {level} is out of allowed range [0, 5]")]
pub struct SkillLevelError {
    pub level: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillLevel {
    pub(crate) fn from_f64_rounded(level: f64) -> Self {
        Self(level.clamp(0.0, 5.0).round() as u8)
    }
    pub(crate) fn from_a_skill_level(a_skill_level: ASkillLevel) -> Self {
        Self(a_skill_level.into_u8())
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0 as f64)
    }
}
impl From<SkillLevel> for u8 {
    fn from(v: SkillLevel) -> Self {
        v.0
    }
}
impl From<SkillLevel> for i32 {
    fn from(v: SkillLevel) -> Self {
        v.0 as i32
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    impl FromStr for SkillLevel {
        type Err = SkillLevelParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let value = i32::from_str(s)?;
            let value = Self::from_i32_checked(value)?;
            Ok(value)
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum SkillLevelParseError {
        #[error("{0}")]
        InvalidInt(#[from] std::num::ParseIntError),
        #[error("{0}")]
        InitCheckFailed(#[from] SkillLevelError),
    }

    impl<'de> serde::Deserialize<'de> for SkillLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            i32::deserialize(deserializer)
                .and_then(|value| SkillLevel::from_i32_checked(value).map_err(serde::de::Error::custom))
        }
    }
}
