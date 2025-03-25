use crate::sol::{FitId, ItemId, ItemTypeId, SkillLevel, uad::item::Skill};

pub struct SkillInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl From<&Skill> for SkillInfo {
    fn from(sol_skill: &Skill) -> Self {
        SkillInfo {
            id: sol_skill.get_item_id(),
            type_id: sol_skill.get_a_item_id(),
            fit_id: sol_skill.get_fit_id(),
            level: sol_skill.get_a_level(),
            enabled: sol_skill.get_skill_state(),
        }
    }
}
