use crate::{
    defs::{EItemId, SkillLevel, SolFitId, SolItemId},
    sol::item::SolSkill,
};

pub struct SolSkillInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SolSkillInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, level: SkillLevel, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            level,
            enabled,
        }
    }
}
impl From<&SolSkill> for SolSkillInfo {
    fn from(sol_skill: &SolSkill) -> Self {
        SolSkillInfo::new(
            sol_skill.get_id(),
            sol_skill.get_type_id(),
            sol_skill.get_fit_id(),
            sol_skill.get_level(),
            sol_skill.get_bool_state(),
        )
    }
}
