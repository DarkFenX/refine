use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::Character},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_character(&mut self, fit_id: &FitId) -> Result<(), RemoveFitCharacterError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.remove_fit_character_internal(fit_key)?)
    }
    pub(in crate::sol) fn remove_fit_character_internal(&mut self, fit_key: FitKey) -> Result<(), FitHasItemKindError> {
        let fit = self.uad.fits.get(fit_key);
        let item_key = match fit.character {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: fit.id,
                    item_kind: Character::get_name(),
                });
            }
        };
        self.remove_character_internal(item_key).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitCharacterError {
    FitNotFound(FitFoundError),
    FitHasNoCharacter(FitHasItemKindError),
}
impl std::error::Error for RemoveFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoCharacter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoCharacter(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for RemoveFitCharacterError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoCharacter(error)
    }
}
