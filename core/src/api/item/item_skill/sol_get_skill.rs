use crate::{
    api::{Skill, SkillMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_skill(&self, item_id: &ItemId) -> Result<Skill<'_>, GetSkillError> {
        let skill_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(skill_uid).dc_skill()?;
        Ok(Skill::new(self, skill_uid))
    }
    pub fn get_skill_mut(&mut self, item_id: &ItemId) -> Result<SkillMut<'_>, GetSkillError> {
        let skill_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(skill_uid).dc_skill()?;
        Ok(SkillMut::new(self, skill_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSkillError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
