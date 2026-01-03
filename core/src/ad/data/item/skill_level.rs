#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ASkillLevel(u8);
impl ASkillLevel {
    pub fn new_clamped_i32(level: i32) -> Self {
        Self(level.clamp(0, 5) as u8)
    }
    pub fn into_inner(self) -> u8 {
        self.0
    }
}
impl std::fmt::Display for ASkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
