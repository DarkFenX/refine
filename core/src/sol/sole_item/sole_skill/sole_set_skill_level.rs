use crate::{
    defs::{SkillLevel, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, SkillLevelError},
    sol::SolarSystem,
};

use super::check_skill_level;

impl SolarSystem {
    pub fn set_skill_level(&mut self, item_id: &SolItemId, level: SkillLevel) -> Result<(), SetSkillLevelError> {
        check_skill_level(level)?;
        let skill = self.uad.items.get_item_mut(item_id)?.get_skill_mut()?;
        skill.set_level(level);
        self.uad
            .fits
            .get_fit_mut(&skill.get_fit_id())
            .unwrap()
            .skills
            .get_mut(&skill.get_type_id())
            .unwrap()
            .level = level;
        let skill = self.uad.items.get_item(item_id).unwrap().get_skill().unwrap();
        self.svc.skill_level_changed(&self.uad, skill);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSkillLevelError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
    SkillLevelError(SkillLevelError),
}
impl std::error::Error for SetSkillLevelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
            Self::SkillLevelError(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSkillLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
            Self::SkillLevelError(e) => e.fmt(f),
        }
    }
}
impl From<SkillLevelError> for SetSkillLevelError {
    fn from(error: SkillLevelError) -> Self {
        Self::SkillLevelError(error)
    }
}
impl From<ItemFoundError> for SetSkillLevelError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSkillLevelError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
