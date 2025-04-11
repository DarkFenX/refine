use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::UadCharacter},
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
                    item_kind: UadCharacter::get_name(),
                });
            }
        };
        self.remove_character_internal(item_key).unwrap();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFitCharacterError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitHasNoCharacter(#[from] FitHasItemKindError),
}
