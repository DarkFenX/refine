use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::CharacterInfo},
};

impl SolarSystem {
    pub fn get_fit_character(&self, fit_id: &FitId) -> Result<Option<CharacterInfo>, GetFitCharacterError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_character_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_character_internal(&self, fit_key: FitKey) -> Option<CharacterInfo> {
        self.uad
            .fits
            .get(fit_key)
            .character
            .map(|item_key| self.get_character_internal(item_key).unwrap())
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
