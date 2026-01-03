use crate::ad;

const LVL_MIN: i32 = 0;
const LVL_MAX: i32 = 5;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SkillLevel(i32);
impl SkillLevel {
    pub fn new_checked(level: impl Into<i32>) -> Result<Self, SkillLevelError> {
        let level = level.into();
        match (LVL_MIN..=LVL_MAX).contains(&level) {
            true => Ok(Self(level)),
            false => Err(SkillLevelError { level }),
        }
    }
    pub fn new_clamped(level: impl Into<i32>) -> Self {
        Self(i32::clamp(level.into(), LVL_MIN, LVL_MAX))
    }
    pub fn get_inner(&self) -> i32 {
        self.0
    }
}
impl std::fmt::Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
// Conversion
impl From<ad::ASkillLevel> for SkillLevel {
    fn from(a_skill_level: ad::ASkillLevel) -> Self {
        Self(a_skill_level.into_inner())
    }
}
impl From<SkillLevel> for ad::ASkillLevel {
    fn from(skill_level: SkillLevel) -> Self {
        Self::new_clamped_i32(skill_level.0)
    }
}
// Equality/ordering
impl PartialEq<ad::ASkillLevel> for SkillLevel {
    fn eq(&self, other: &ad::ASkillLevel) -> bool {
        self.get_inner().eq(&other.into_inner())
    }
}
impl PartialEq<SkillLevel> for ad::ASkillLevel {
    fn eq(&self, other: &SkillLevel) -> bool {
        self.into_inner().eq(&other.get_inner())
    }
}
impl PartialOrd<ad::ASkillLevel> for SkillLevel {
    fn partial_cmp(&self, other: &ad::ASkillLevel) -> Option<std::cmp::Ordering> {
        self.get_inner().partial_cmp(&other.into_inner())
    }
}
impl PartialOrd<SkillLevel> for ad::ASkillLevel {
    fn partial_cmp(&self, other: &SkillLevel) -> Option<std::cmp::Ordering> {
        self.into_inner().partial_cmp(&other.get_inner())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("skill level {level} is out of allowed range [0, 5]")]
pub struct SkillLevelError {
    pub level: i32,
}
