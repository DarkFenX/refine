use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::CharacterInfo},
};

impl SolarSystem {
    pub fn get_fit_character(&self, fit_id: &FitId) -> Result<Option<CharacterInfo>, GetFitCharacterError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(fit
            .character
            .map(|item_key| self.get_character_internal(item_key).unwrap()))
    }
}

#[derive(Debug)]
pub enum GetFitCharacterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
