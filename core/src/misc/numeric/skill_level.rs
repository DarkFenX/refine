use crate::{ad::ASkillLevel, misc::Value};

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
