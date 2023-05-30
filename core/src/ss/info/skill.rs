use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Skill,
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
impl From<&Skill> for SkillInfo {
    fn from(s: &Skill) -> Self {
        SkillInfo::new(s.id, s.fit_id, s.type_id, s.level, s.get_bool_state())
    }
}
