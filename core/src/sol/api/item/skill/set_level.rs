use crate::{
    err::basic::SkillLevelError,
    sol::{ItemKey, SkillLevel, SolarSystem, api::SkillMut},
};

use super::misc::check_skill_level;

impl SolarSystem {
    pub(in crate::sol) fn internal_set_skill_level(&mut self, item_key: ItemKey, level: SkillLevel) {
        let uad_skill = self.uad.items.get_mut(item_key).get_skill_mut().unwrap();
        if uad_skill.get_a_level() == level {
            return;
        }
        uad_skill.set_a_level(level);
        self.uad
            .fits
            .get_mut(uad_skill.get_fit_key())
            .skills
            .get_mut(&uad_skill.get_a_item_id())
            .unwrap()
            .level = level;
        let uad_skill = self.uad.items.get(item_key).get_skill().unwrap();
        self.svc.skill_level_changed(&self.uad, item_key, uad_skill);
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_level(&mut self, level: SkillLevel) -> Result<(), SetSkillLevelError> {
        check_skill_level(level)?;
        self.sol.internal_set_skill_level(self.key, level);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSkillLevelError {
    #[error("{0}")]
    SkillLevelError(#[from] SkillLevelError),
}
