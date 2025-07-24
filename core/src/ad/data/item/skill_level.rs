#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ASkillLevel {
    inner: i32,
}
impl ASkillLevel {
    pub fn new(level: impl Into<i32>) -> Self {
        Self {
            inner: i32::clamp(level.into(), 0, 5),
        }
    }
    pub fn get_inner(&self) -> i32 {
        self.inner
    }
}
impl std::fmt::Display for ASkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
