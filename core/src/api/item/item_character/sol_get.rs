use crate::{
    api::{Character, CharacterMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_character(&self, item_id: &ItemId) -> Result<Character<'_>, GetCharacterError> {
        let character_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(character_uid).dc_character()?;
        Ok(Character::new(self, character_uid))
    }
    pub fn get_character_mut(&mut self, item_id: &ItemId) -> Result<CharacterMut<'_>, GetCharacterError> {
        let character_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(character_uid).dc_character()?;
        Ok(CharacterMut::new(self, character_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetCharacterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
