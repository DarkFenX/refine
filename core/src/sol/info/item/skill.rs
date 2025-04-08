use crate::sol::{FitId, ItemId, ItemTypeId, SkillLevel, uad::item::Skill};

pub struct SkillInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SkillInfo {
    pub(in crate::sol) fn from_skill(skill: &Skill) -> Self {
        Self {
            id: skill.get_item_id(),
            type_id: skill.get_a_item_id(),
            fit_id: skill.get_fit_id(),
            level: skill.get_a_level(),
            enabled: skill.get_skill_state(),
        }
    }
}
