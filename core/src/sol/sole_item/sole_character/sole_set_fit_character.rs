use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::CharacterInfo,
        uad::item::{Character, Item},
    },
};

impl SolarSystem {
    pub fn set_fit_character(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<CharacterInfo, SetFitCharacterError> {
        let fit = self.uad.fits.get_fit(&fit_id)?;
        // Remove old character, if it was set
        if let Some(old_item_id) = fit.character {
            self.remove_item_id_from_svc(&old_item_id);
            self.uad.items.remove_by_id(&old_item_id);
        }
        // Add new character
        let item_id = self.uad.items.alloc_item_id();
        let character = Character::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = CharacterInfo::from(&character);
        let item = Item::Character(character);
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.character = Some(item_id);
        self.uad.items.add(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum SetFitCharacterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
