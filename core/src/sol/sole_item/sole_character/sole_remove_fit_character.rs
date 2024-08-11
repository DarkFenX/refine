use crate::{
    defs::SolFitId,
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{item::SolCharacter, SolarSystem},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_character(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitCharacterError> {
        let fit = self.fits.get_fit(fit_id)?;
        let item_id = match fit.character {
            Some(item_id) => item_id,
            None => return Err(FitHasItemKindError::new(*fit_id, SolCharacter::get_name()).into()),
        };
        self.remove_character(&item_id).unwrap();
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
