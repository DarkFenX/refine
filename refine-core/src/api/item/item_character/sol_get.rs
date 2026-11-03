use crate::{
    Character, ItemId, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

impl SolarSystem {
    pub fn get_character(&mut self, item_id: &ItemId) -> Result<Character<'_>, CharacterGetError> {
        let character_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(character_uid).dc_character()?;
        Ok(Character::new(self, character_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CharacterGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
