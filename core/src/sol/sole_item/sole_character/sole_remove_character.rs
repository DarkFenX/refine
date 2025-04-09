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

#[derive(thiserror::Error, Debug)]
pub enum RemoveCharacterError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotCharacter(#[from] ItemKindMatchError),
}
