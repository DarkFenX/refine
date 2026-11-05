use crate::{
    ItemId, Skill, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_skill(&mut self, item_id: &ItemId) -> Result<Skill<'_>, SkillGetError> {
        let sol = self.inner_mut();
        let skill_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(skill_uid).dc_skill()?;
        Ok(Skill::new(sol, skill_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SkillGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
