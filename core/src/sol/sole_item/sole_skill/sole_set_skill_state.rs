use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn set_skill_state(&mut self, item_id: &ItemId, state: bool) -> Result<(), SetSkillStateError> {
        let skill = self.uad.items.get_mut_by_id(item_id)?.get_skill_mut()?;
        let old_a_state = skill.get_a_state();
        skill.set_skill_state(state);
        let new_a_state = skill.get_a_state();
        self.change_item_id_state_in_svc(item_id, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSkillStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
}
impl std::error::Error for SetSkillStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSkillStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetSkillStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSkillStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
