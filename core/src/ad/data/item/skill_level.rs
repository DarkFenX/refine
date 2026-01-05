#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ASkillLevel(u8);
impl ASkillLevel {
    pub fn new_clamped_i32(level: i32) -> Self {
        Self(level.clamp(0, 5) as u8)
    }
    pub fn into_inner(self) -> u8 {
        self.0
    }
}
