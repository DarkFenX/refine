use crate::ad;

pub(in crate::sol) type SkillLevelInner = i8;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SkillLevel {
    inner: SkillLevelInner,
}
impl SkillLevel {
    pub fn new_checked(level: impl Into<SkillLevelInner>) -> Result<Self, SkillLevelError> {
        match level.into() {
            level @ ..=-1 | level @ 6.. => Err(SkillLevelError { level }),
            level => Ok(Self { inner: level }),
        }
    }
    pub fn new_clamped(level: impl Into<SkillLevelInner>) -> Self {
        Self {
            inner: 0.max(5.min(level.into())),
        }
    }
    pub fn get_inner(&self) -> SkillLevelInner {
        self.inner
    }
}
impl std::fmt::Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
// Conversion
impl From<ad::ASkillLevel> for SkillLevel {
    fn from(a_skill_level: ad::ASkillLevel) -> Self {
        Self {
            inner: a_skill_level.get_inner(),
        }
    }
}
impl From<SkillLevel> for ad::ASkillLevel {
    fn from(skill_level: SkillLevel) -> Self {
        Self::new(skill_level.inner)
    }
}
// Equality/ordering
impl PartialEq<ad::ASkillLevel> for SkillLevel {
    fn eq(&self, other: &ad::ASkillLevel) -> bool {
        self.get_inner().eq(&other.get_inner())
    }
}
impl PartialEq<SkillLevel> for ad::ASkillLevel {
    fn eq(&self, other: &SkillLevel) -> bool {
        self.get_inner().eq(&other.get_inner())
    }
}
impl PartialOrd<ad::ASkillLevel> for SkillLevel {
    fn partial_cmp(&self, other: &ad::ASkillLevel) -> Option<std::cmp::Ordering> {
        self.get_inner().partial_cmp(&other.get_inner())
    }
}
impl PartialOrd<SkillLevel> for ad::ASkillLevel {
    fn partial_cmp(&self, other: &SkillLevel) -> Option<std::cmp::Ordering> {
        self.get_inner().partial_cmp(&other.get_inner())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("skill level {level} is out of allowed range [0, 5]")]
pub struct SkillLevelError {
    pub level: i8,
}
