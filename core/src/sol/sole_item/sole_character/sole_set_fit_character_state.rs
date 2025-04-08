use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::Character},
    util::Named,
};

impl SolarSystem {
    pub fn set_fit_character_state(&mut self, fit_id: &FitId, state: bool) -> Result<(), SetFitCharacterStateError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.set_fit_character_state_internal(fit_key, state)?)
    }
    pub(in crate::sol) fn set_fit_character_state_internal(
        &mut self,
        fit_key: FitKey,
        state: bool,
    ) -> Result<(), FitHasItemKindError> {
        let fit = self.uad.fits.get(fit_key);
        let item_key = match fit.character {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: fit.id,
                    item_kind: Character::get_name(),
                }
                .into());
            }
        };
        let character = self.uad.items.get_mut(item_key).get_character_mut().unwrap();
        let old_a_state = character.get_a_state();
        character.set_character_state(state);
        let new_a_state = character.get_a_state();
        self.change_item_key_state_in_svc(item_key, old_a_state, new_a_state);
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitCharacterStateError {
    FitNotFound(FitFoundError),
    FitHasNoCharacter(FitHasItemKindError),
}
impl std::error::Error for SetFitCharacterStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoCharacter(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitCharacterStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoCharacter(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitCharacterStateError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for SetFitCharacterStateError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoCharacter(error)
    }
}
