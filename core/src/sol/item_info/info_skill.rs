use crate::{
    defs::{EItemId, SkillLevel, SolFitId, SolItemId},
    sol::item::SolSkill,
};

pub struct SolSkillInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SolSkillInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, level: SkillLevel, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            level,
            enabled,
        }
    }
}
impl From<&SolSkill> for SolSkillInfo {
    fn from(sol_skill: &SolSkill) -> Self {
        SolSkillInfo::new(
            sol_skill.base.id,
            sol_skill.fit_id,
            sol_skill.base.a_item_id,
            sol_skill.level,
            sol_skill.get_bool_state(),
        )
    }
}
