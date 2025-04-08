use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_character(&mut self, item_id: &ItemId) -> Result<(), RemoveCharacterError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_character_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_character_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let character = item.get_character()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(character.get_fit_key());
        fit.character = None;
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveCharacterError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharacter(ItemKindMatchError),
}
impl std::error::Error for RemoveCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharacter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharacter(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveCharacterError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveCharacterError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharacter(error)
    }
}
