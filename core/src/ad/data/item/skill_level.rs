#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ASkillLevel(u8);
impl ASkillLevel {
    pub fn from_i32_clamped(level: i32) -> Self {
        Self(level.clamp(0, 5) as u8)
    }
    pub fn into_u8(self) -> u8 {
        self.0
    }
}
