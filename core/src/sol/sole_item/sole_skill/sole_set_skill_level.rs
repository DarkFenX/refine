use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, SkillLevelError},
    sol::{ItemId, ItemKey, SkillLevel, SolarSystem},
};

use super::check_skill_level;

impl SolarSystem {
    pub fn set_skill_level(&mut self, item_id: &ItemId, level: SkillLevel) -> Result<(), SetSkillLevelError> {
        check_skill_level(level)?;
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.set_skill_level_internal(item_key, level)?)
    }
    pub(in crate::sol) fn set_skill_level_internal(
        &mut self,
        item_key: ItemKey,
        level: SkillLevel,
    ) -> Result<(), ItemKindMatchError> {
        let skill = self.uad.items.get_mut(item_key).get_skill_mut()?;
        if skill.get_a_level() == level {
            return Ok(());
        }
        skill.set_a_level(level);
        self.uad
            .fits
            .get_mut(skill.get_fit_key())
            .skills
            .get_mut(&skill.get_a_item_id())
            .unwrap()
            .level = level;
        let skill = self.uad.items.get(item_key).get_skill().unwrap();
        self.svc.skill_level_changed(&self.uad, item_key, skill);
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
impl From<SkillLevelError> for SetSkillLevelError {
    fn from(error: SkillLevelError) -> Self {
        Self::SkillLevelError(error)
    }
}
