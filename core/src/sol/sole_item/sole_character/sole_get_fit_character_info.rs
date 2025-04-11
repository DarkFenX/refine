use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::CharacterInfo},
};

impl SolarSystem {
    pub fn get_fit_character_info(&self, fit_id: &FitId) -> Result<Option<CharacterInfo>, GetFitCharacterInfoError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_character_info_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_character_info_internal(&self, fit_key: FitKey) -> Option<CharacterInfo> {
        self.uad
            .fits
            .get(fit_key)
            .character
            .map(|item_key| self.get_character_info_internal(item_key).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitCharacterInfoError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
