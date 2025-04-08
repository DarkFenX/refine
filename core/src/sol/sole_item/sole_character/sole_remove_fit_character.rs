use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, SolarSystem, uad::item::Character},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_character(&mut self, fit_id: &FitId) -> Result<(), RemoveFitCharacterError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_key = match fit.character {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: *fit_id,
                    item_kind: Character::get_name(),
                }
                .into());
            }
        };
        Ok(self.remove_character_internal(item_key).unwrap())
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
