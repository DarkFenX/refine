use crate::{
    Character, ItemId, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_character(&mut self, item_id: &ItemId) -> Result<Character<'_>, CharacterGetError> {
        let core = self.inner_mut();
        let character_uid = core.u_data.items.int_id_by_ext_id_err(item_id)?;
        core.u_data.items.get(character_uid).dc_character()?;
        Ok(Character::new(core, character_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CharacterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
