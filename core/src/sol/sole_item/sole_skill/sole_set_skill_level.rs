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

#[derive(thiserror::Error, Debug)]
pub enum SetSkillLevelError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
    #[error("{0}")]
    SkillLevelError(#[from] SkillLevelError),
}
