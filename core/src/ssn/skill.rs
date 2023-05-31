use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SkillInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub level: ReeInt,
    pub enabled: bool,
}
impl SkillInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            level,
            enabled,
        }
    }
}
impl From<&ssi::Skill> for SkillInfo {
    fn from(s: &ssi::Skill) -> Self {
        SkillInfo::new(s.id, s.fit_id, s.type_id, s.level, s.get_bool_state())
    }
}
