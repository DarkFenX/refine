use crate::sol::{
    FitId, ItemId, ItemTypeId, SkillLevel,
    uad::{Uad, item::Skill},
};

pub struct SkillInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub level: SkillLevel,
    pub enabled: bool,
}
impl SkillInfo {
    pub(in crate::sol) fn from_skill(uad: &Uad, skill: &Skill) -> Self {
        Self {
            id: skill.get_item_id(),
            type_id: skill.get_a_item_id(),
            fit_id: uad.fits.id_by_key(skill.get_fit_key()),
            level: skill.get_a_level(),
            enabled: skill.get_skill_state(),
        }
    }
}
