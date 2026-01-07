use crate::{
    api::{Character, CharacterMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_character(&self, item_id: &ItemId) -> Result<Character<'_>, GetCharacterError> {
        let character_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(character_key).dc_character()?;
        Ok(Character::new(self, character_key))
    }
    pub fn get_character_mut(&mut self, item_id: &ItemId) -> Result<CharacterMut<'_>, GetCharacterError> {
        let character_key = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(character_key).dc_character()?;
        Ok(CharacterMut::new(self, character_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetCharacterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
