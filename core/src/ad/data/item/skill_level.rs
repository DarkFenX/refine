pub(in crate::ad) type ASkillLevelInner = i8;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ASkillLevel {
    inner: ASkillLevelInner,
}
impl ASkillLevel {
    pub fn new(level: impl Into<ASkillLevelInner>) -> Self {
        Self {
            inner: ASkillLevelInner::max(0, ASkillLevelInner::min(5, level.into())),
        }
    }
    pub fn get_inner(&self) -> ASkillLevelInner {
        self.inner
    }
}
impl std::fmt::Display for ASkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
