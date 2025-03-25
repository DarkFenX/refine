use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_character(&mut self, item_id: &ItemId) -> Result<(), RemoveCharacterError> {
        let item = self.uad.items.get_item(item_id)?;
        let character = item.get_character()?;
        self.svc.remove_item(&self.uad, item);
        let fit = self.uad.fits.get_fit_mut(&character.get_fit_id()).unwrap();
        fit.character = None;
        self.uad.items.remove_item(item_id);
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
