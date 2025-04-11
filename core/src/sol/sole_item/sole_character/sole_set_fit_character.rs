use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::CharacterInfo,
        uad::item::{UadCharacter, UadItem},
    },
};

impl SolarSystem {
    pub fn set_fit_character(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<CharacterInfo, SetFitCharacterError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.set_fit_character_internal(fit_key, type_id, state);
        Ok(self.get_character_info_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn set_fit_character_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let fit = self.uad.fits.get(fit_key);
        // Remove old character, if it was set
        if let Some(old_item_key) = fit.character {
            self.remove_character_internal(old_item_key).unwrap();
        }
        // Add new character
        let item_id = self.uad.items.alloc_id();
        let character = UadCharacter::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = UadItem::Character(character);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.character = Some(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitCharacterError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
