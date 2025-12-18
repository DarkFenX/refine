use crate::{
    api::{Skill, SkillMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_skill(&self, item_id: &ItemId) -> Result<Skill<'_>, GetSkillError> {
        let skill_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(skill_key).dc_skill()?;
        Ok(Skill::new(self, skill_key))
    }
    pub fn get_skill_mut(&mut self, item_id: &ItemId) -> Result<SkillMut<'_>, GetSkillError> {
        let skill_key = self.u_data.items.key_by_id_err(item_id)?;
        self.u_data.items.get(skill_key).dc_skill()?;
        Ok(SkillMut::new(self, skill_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSkillError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
