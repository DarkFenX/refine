use crate::{
    defs::{EItemId, SkillLevel, SsFitId, SsItemId},
    ss::item::SsSkill,
};

pub struct SsSkillInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SsSkillInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, level: SkillLevel, enabled: bool) -> Self {
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
