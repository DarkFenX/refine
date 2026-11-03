use crate::{
    api::Skill,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_skill(&mut self, item_id: &ItemId) -> Result<Skill<'_>, SkillGetError> {
        let skill_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(skill_uid).dc_skill()?;
        Ok(Skill::new(self, skill_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SkillGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
