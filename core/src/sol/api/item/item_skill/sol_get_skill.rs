use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Skill, SkillMut},
    },
};

impl SolarSystem {
    pub fn get_skill(&self, item_id: &ItemId) -> Result<Skill<'_>, GetSkillError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_skill()?;
        Ok(Skill::new(self, item_key))
    }
    pub fn get_skill_mut(&mut self, item_id: &ItemId) -> Result<SkillMut<'_>, GetSkillError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_skill()?;
        Ok(SkillMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSkillError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
