use crate::{
    defs::{EItemId, SkillLevel, SolFitId, SolItemId},
    sol::item::SolSkill,
};

pub struct SolSkillInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SolSkillInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, level: SkillLevel, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            level,
            enabled,
        }
    }
}
impl From<&SolSkill> for SolSkillInfo {
    fn from(sol_skill: &SolSkill) -> Self {
        SolSkillInfo::new(
            sol_skill.get_id(),
            sol_skill.get_fit_id(),
            sol_skill.get_type_id(),
            sol_skill.get_level(),
            sol_skill.get_bool_state(),
        )
    }
}
