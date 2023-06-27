use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsSkill,
};

pub struct SsSkillInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub level: ReeInt,
    pub enabled: bool,
}
impl SsSkillInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, level: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            level,
            enabled,
        }
    }
}
impl From<&SsSkill> for SsSkillInfo {
    fn from(ss_skill: &SsSkill) -> Self {
        SsSkillInfo::new(
            ss_skill.id,
            ss_skill.fit_id,
            ss_skill.a_item_id,
            ss_skill.level,
            ss_skill.get_bool_state(),
        )
    }
}
